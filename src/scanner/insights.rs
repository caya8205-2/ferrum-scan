use super::RepoStats;

pub fn generate_insights(stats: &RepoStats) -> Vec<String> {
    let mut insights = Vec::new();

    // 1. Dominant Language Insight
    if let Some(top_lang) = stats.languages.first() {
        let share = if stats.total_code_lines > 0 {
            (top_lang.code_lines as f64 / stats.total_code_lines as f64) * 100.0
        } else {
            0.0
        };
        insights.push(format!(
            "Dominant Language: {} ({:.1}% of code LOC).",
            top_lang.name, share
        ));
    }

    // 2. Documentation Ratio Insight
    let comment_percentage = if stats.total_lines > 0 {
        (stats.total_comment_lines as f64 / stats.total_lines as f64) * 100.0
    } else {
        0.0
    };

    if comment_percentage < 5.0 {
        insights.push(format!(
            "Low documentation ratio ({:.1}% comments). Consider adding doc comments.",
            comment_percentage
        ));
    } else if comment_percentage >= 15.0 {
        insights.push(format!(
            "Strong documentation ratio ({:.1}% comments).",
            comment_percentage
        ));
    } else {
        insights.push(format!(
            "Moderate documentation ratio ({:.1}% comments).",
            comment_percentage
        ));
    }

    // 3. Storage & Wasted Space
    if stats.total_wasted_bytes > 1_000_000 {
        let wasted_mb = stats.total_wasted_bytes as f64 / (1024.0 * 1024.0);
        insights.push(format!(
            "Duplicate bloat detected: {:.2} MB of wasted storage space can be reclaimed.",
            wasted_mb
        ));
    }

    // 4. Jumbo Files Warning
    if !stats.jumbo_files.is_empty() {
        let total_jumbo_size_mb: f64 = stats.jumbo_files.iter().map(|j| j.size_mb).sum();
        insights.push(format!(
            "Jumbo files alert: {} file(s) exceed size threshold (total {:.2} MB).",
            stats.jumbo_files.len(),
            total_jumbo_size_mb
        ));
    }

    // 5. Git Status Insight
    if stats.git_health.is_git_repo {
        if let Some(ref date) = stats.git_health.last_commit_date {
            insights.push(format!(
                "Git Repository active with {} commits across {} contributor(s) (Last commit: {}).",
                stats.git_health.commit_count, stats.git_health.total_contributors, date
            ));
        } else {
            insights.push(format!(
                "Git Repository tracked ({} commits, {} contributor(s)).",
                stats.git_health.commit_count, stats.git_health.total_contributors
            ));
        }
    } else {
        insights.push("Not a Git repository. Initialize git to track repository history.".to_string());
    }

    // 6. Overall Health Grade
    let grade = calculate_health_grade(stats, comment_percentage);
    insights.push(format!("Overall Repository Health Rating: {}", grade));

    insights
}

fn calculate_health_grade(stats: &RepoStats, comment_pct: f64) -> &'static str {
    let mut points = 100i32;

    if !stats.git_health.is_git_repo {
        points -= 15;
    }
    if comment_pct < 5.0 {
        points -= 10;
    }
    if !stats.jumbo_files.is_empty() {
        points -= 15;
    }
    if stats.total_wasted_bytes > 5_000_000 {
        points -= 15;
    }

    if points >= 90 {
        "A+ (Optimal & Clean)"
    } else if points >= 80 {
        "A (Good Condition)"
    } else if points >= 70 {
        "B (Needs Minor Cleanup)"
    } else if points >= 60 {
        "C (Requires Optimization)"
    } else {
        "D (Bloated / Unmaintained)"
    }
}
