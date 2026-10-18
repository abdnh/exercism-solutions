use std::{collections::HashMap, fmt::Write, str::FromStr};

struct TeamStats {
    wins: usize,
    losses: usize,
    draws: usize,
}

impl Default for TeamStats {
    fn default() -> Self {
        Self {
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }
}

impl TeamStats {
    fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl FromStr for MatchResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            _ => return Err(()),
        })
    }
}

pub fn tally(match_results: &str) -> String {
    let mut stats: HashMap<String, TeamStats> = HashMap::new();
    for line in match_results.lines() {
        let parts: Vec<&str> = line.split(';').collect();
        let Ok([team1, team2, result]) =
            <&[&str] as TryInto<[&str; 3]>>::try_into(parts.as_slice())
        else {
            continue;
        };
        let Ok(result) = result.parse::<MatchResult>() else {
            continue;
        };
        for (team, first) in [(team1, true), (team2, false)] {
            let team_stats = stats.entry(team.into()).or_default();
            match result {
                MatchResult::Win => {
                    if first {
                        team_stats.wins += 1;
                    } else {
                        team_stats.losses += 1;
                    }
                }
                MatchResult::Loss => {
                    if first {
                        team_stats.losses += 1;
                    } else {
                        team_stats.wins += 1;
                    }
                }
                MatchResult::Draw => {
                    team_stats.draws += 1;
                }
            }
        }
    }

    let mut output = String::new();
    let mut stats: Vec<(String, TeamStats)> = stats.drain().collect();
    stats.sort_by(|a, b| a.0.cmp(&b.0));
    stats.sort_by(|a, b| b.1.points().cmp(&a.1.points()));
    write!(
        output,
        "Team                           | MP |  W |  D |  L |  P"
    )
    .unwrap();
    for (team, team_stats) in stats {
        let wins = team_stats.wins;
        let losses = team_stats.losses;
        let draws = team_stats.draws;
        let total = wins + losses + draws;
        let points = team_stats.points();
        write!(
            output,
            "\n{team:30} | {total:>2} | {wins:>2} | {draws:>2} | {losses:>2} | {points:>2}"
        )
        .unwrap();
    }

    output
}
