# 🏰 Rust Adventure: The Secrets of Functions ⚔️📜

The **Kingdom of Rustoria** is in trouble! The **Royal Wizard** has lost his spells, the **Blacksmith** needs help crafting weapons, and the **Healers** are struggling to heal wounded warriors. Only a true **Rustacean** can help by restoring the kingdom's **functions**!  

Your **mission**: Solve these **10 challenges** by writing **functions in Rust**!  

---

## ⚔️ Quest 1: The Lost Battle Cry  
The knights of Rustoria are preparing for war, but they need a **battle cry function**!  

🔹 **Task:** Write a function `battle_cry()` that prints:  
📢 `"For Rustoria! Charge! ⚔️"`  

💡 **Hint:** Use a simple function without parameters.  

---

## 🔥 Quest 2: The Blacksmith’s Sword  
The Blacksmith needs to craft a **custom sword** for the king! The **sword's power** depends on the quality of the metal.  

🔹 **Task:** Write a function `craft_sword(metal_quality: i32) -> i32` that **doubles** the metal quality and returns the **sword's power**.  

💡 **Example:** If `metal_quality = 10`, the function should return `20`.  

---

## 🧙‍♂️ Quest 3: The Wizard’s Fireball  
The Royal Wizard forgot his **fireball spell**! Help him by creating a **function** that casts a fireball with a given strength.  

🔹 **Task:** Write `cast_fireball(power: i32) -> String` that returns:  
🧙‍♂️ `"🔥 Fireball of power X!"`  

💡 **Example:** If power = `50`, the output should be `"🔥 Fireball of power 50!"`  

---

## 🏥 Quest 4: The Healer’s Potion  
The kingdom’s **healers** need a function to **restore health** to warriors!  

🔹 **Task:** Write `heal_warrior(current_health: i32, potion_power: i32) -> i32` that adds `potion_power` to `current_health` and returns the new health value.  

💡 **Example:** If a warrior has `50` health and drinks a potion of `30`, they should have `80` health.  

---

## 🏹 Quest 5: The Archer’s Arrow Count  
A group of archers is heading into battle, but they need to know how many **arrows they have left**.  

🔹 **Task:** Write a function `arrows_left(total: i32, used: i32) -> i32` that subtracts `used` arrows from the `total` and returns the remaining arrows.  

💡 **Example:** If they had `100` arrows and used `25`, the function should return `75`.  

---

## 🏛️ Quest 6: The Merchant’s Tax Calculator  
The **Royal Merchant** needs a **tax calculator** for his business!  

🔹 **Task:** Write a function `calculate_tax(price: f64, tax_rate: f64) -> f64` that calculates the total price after tax.  

💡 **Example:** If `price = 100.0` and `tax_rate = 0.15`, the function should return `115.0`.  

---

## ⚖️ Quest 7: The King’s Judgment  
The King of Rustoria must decide the fate of criminals based on their crimes.  

🔹 **Task:** Write `judge_criminal(crime: &str) -> &str` that returns:  
- `"⚖️ Punishment: Jail!"` for "theft"  
- `"⚔️ Punishment: Exile!"` for "treason"  
- `"🕊 Mercy: Set free!"` for any other case.  

💡 **Hint:** Use an `if-else` statement.  

---

## 🎭 Quest 8: The Jester’s Performance  
The **Royal Jester** tells jokes, but he needs to know how many times he should perform.  

🔹 **Task:** Write `jester_performance(times: i32) {}` that prints `"🤹 Jester performs!"` for the given number of times.  

💡 **Example:** If `times = 3`, print the message **3 times**.  

---

## 🐉 Quest 9: The Dragon’s Power  
The **Elder Dragon** of Rustoria has **3** power levels: "sleeping", "angry", and "raging".  

🔹 **Task:** Write `dragon_power(level: i32) -> &str` that returns:  
- `"🐉 The dragon is sleeping..."` for `level = 1`  
- `"🔥 The dragon is angry!"` for `level = 2`  
- `"💀 The dragon is raging! Run!"` for `level = 3`  
- Any other value should return `"❓ Unknown dragon state."`  

💡 **Hint:** Use a `match` statement.  

---

## 👑 Quest 10: The Crown’s Jewel  
The King wants a **function that checks if a number is prime** to find the **Crown’s Jewel** (a prime number).  

🔹 **Task:** Write `is_prime(n: i32) -> bool` that returns `true` if `n` is prime and `false` otherwise.  

💡 **Example:** `is_prime(7) -> true`, `is_prime(10) -> false`  

---

## 🎯 Your Final Challenge!  

- Write Rust programs for each **quest**.
- Save them as `exercise.rs`.
- Test your code and share your results!  

⚔️ **Brave adventurer, your journey begins now!** 🚀

