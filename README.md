Task 5
Do a loop where you either attack the main enemy, get a new defense multiplier or use a potion.
Until you - the player, or the main enemy is destroyed, the loop continues.

If you attack the main enemy, use the receive_player_attack_dmg function you created.
If you defend yourself, use the receive_defense_multiplier function you created, which will generate a new defense multiplier for you.
If you use a potion and improve your life points, use your own function for this as well. Add 25 points to the player if there are still potions left.

On your turn, do only 1 of the options above. If the main enemy is still alive after your turn, it will attack you and the amount of damage from the attack depends on your defense factor.
Before the battle (the main loop of your program), initialize your potion count to 3, the player's life points to 100, the main enemy's life points to 150. Use float numbers for life points.

Make a function called receive_player_attack_dmg that returns a random number between 12.5 and 20.
Make a function called receive_defense_multiplier that generates a random number between 2 and 4, and returns a value that is 1 / generated_random_number - i.e. the returned number is less than 1.
Make a function called receive_boss_attack_dmg that returns a random number between 5 and 25.

To generate random numbers, use rand-crate. Import it in the Cargo.toml file.

Esimerkkiajo käyttäjän syötteiden kanssa:

| Your HP - 100 | Boss HP - 150 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 14.957747 amount of damage.
You take 19.060558 damage.
| Your HP - 80.93944 | Boss HP - 135.04225 | 
| 1) Attack | 2) Defend | 3) Heal |
2
Defense activated!
You take 4.502411 damage.
| Your HP - 76.43703 | Boss HP - 135.04225 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 16.456348 amount of damage.
You take 17.453207 damage.
| Your HP - 58.983818 | Boss HP - 118.58591 |
| 1) Attack | 2) Defend | 3) Heal |
3
You consume a potion.
You take 9.422489 damage.
| Your HP - 74.561325 | Boss HP - 118.58591 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 17.22312 amount of damage.
You take 23.554003 damage.
| Your HP - 51.007324 | Boss HP - 101.362785 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.885159 amount of damage.
You take 16.806934 damage.
| Your HP - 34.20039 | Boss HP - 82.47763 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.075796 amount of damage.
You take 14.347532 damage.
| Your HP - 19.852858 | Boss HP - 64.40183 |
| 1) Attack | 2) Defend | 3) Heal |
3
You consume a potion.
You take 24.755264 damage.
| Your HP - 20.097595 | Boss HP - 64.40183 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 18.169785 amount of damage.
You take 5.479629 damage.
| Your HP - 14.617966 | Boss HP - 46.232048 |
| 1) Attack | 2) Defend | 3) Heal |
1
Your attack deals 17.021484 amount of damage.
You take 19.016312 damage.
You have been defeated!
