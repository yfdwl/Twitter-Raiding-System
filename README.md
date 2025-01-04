# Raiding Points System

## Overview

The **Raiding Points System** allows projects to define custom raiding incentives that enable users to earn points through various social media interactions. Projects can configure how many points users will earn for specific actions such as:

- Following a page
- Liking posts
- Reposting tweets dpecPN1Nfg
- Commenting with a specific number of tags

The system tracks the accumulation of points based on these user actions. It allows projects to customize the points awarded for each action and select different types of rewards, such as BRC-20 tokens, Runes, Inscriptions, or off-chain points.

### Key Features

1. **Raiding Points System**
    - Users earn points based on configurable actions like following, liking, reposting, and commenting.
    - Projects can define how many points are awarded for each action.
    - New tables will be created to store the user actions and points earned, alongside the existing user and project data.

2. **Customizable Rewards**
    - Projects can define the type of rewards users can earn based on their accumulated points.
    - Rewards can include on-chain tokens (e.g., BRC-20), runes, inscriptions, or off-chain points.
    - The system itself will handle only the point accrual process, leaving reward distribution as a separate feature handled by other components.

3. **Leaderboards**
    - Users can view leaderboards that show top participants based on the number of points they've earned.
    - Leaderboards are updated as users accumulate points from different interactions.