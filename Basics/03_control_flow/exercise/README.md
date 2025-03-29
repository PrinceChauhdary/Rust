# 🏛️ Advanced Control Flow Quests in Rust 🧟‍♂️🛡️  

## 🏝️ **Quest 1: The Deserted Treasure Chest** 💰  
You are exploring the **Cursed Ruins** when you find a **locked treasure chest**! It has a **combination lock (number between 1-5).**  

- **1** → `✨ The chest unlocks, revealing ancient gold!`  
- **2** → `⚔️ A trap is triggered! You dodge just in time!`  
- **3** → `🗿 The chest is empty... Just an illusion.`  
- **4** → `📝 You find a mysterious map leading to another treasure!`  
- **5** → `🐍 A poisonous snake jumps out! You narrowly escape!`  
- Any other number → `❌ The chest remains locked. Try again!`  

**Challenge:** Use a `match` expression to handle the possible outcomes.  

---  

## 🏹 **Quest 2: The Ranger’s Challenge** 🌲  
A **forest ranger** asks you to **identify an animal** based on the number of legs it has:  

- **2 legs** → `🦅 It's a majestic eagle!`  
- **4 legs** → `🦊 A swift fox dashes through the woods!`  
- **6 legs** → `🐜 An ant carries a leaf three times its size!`  
- **8 legs** → `🕷️ A spider spins its web!`  
- Any other number → `❓ An unknown creature appears!`  

**Challenge:** Use a `match` statement to identify the animal based on leg count.  

---  

## 🏴‍☠️ **Quest 3: The Pirate’s Code** ☠️  
A **pirate captain** gives you **5 gold coins** and tells you to **bet on a number** (`1-3`).  

- If the **random number matches your bet**, print `🏆 You won! The pirate doubles your gold!`  
- If the **random number does not match**, print `💸 You lost! The pirate takes your gold!`  
- If you bet **outside 1-3**, print `❌ The pirate laughs at you and walks away!`  

**Challenge:** Use `if/else` with `rand::random()` to generate a random number.  

---  

## 🐉 **Quest 4: The Dragon’s Riddle** 🔥  
The **Ancient Dragon** tests your wisdom with a **math puzzle**:  

`Tell me a number, and I shall reveal if it is prime!`  

- If the number is **prime**, print `🔥 The dragon nods in approval!`  
- If the number is **not prime**, print `😈 The dragon scoffs and breathes fire!`  

**Challenge:** Write a function that checks if a number is **prime** using a loop and `if/else`.  

---  

## 🎭 **Quest 5: The Festival of Lights** 🎆  
You are in the **Great City of Rustoria** during the **Festival of Lights**! The event has **ticket prices**:  

- **Under 5 years old** → `🍼 Free entry!`  
- **5-12 years old** → `🎟️ Child ticket: 5 gold coins.`  
- **13-60 years old** → `🎟️ Adult ticket: 10 gold coins.`  
- **Above 60 years old** → `🎟️ Senior ticket: 7 gold coins.`  

**Challenge:** Use a `match` statement with **ranges** to determine ticket prices.  

---  

## 🏰 **Quest 6: The Secret Password** 🔑  
You have **three attempts** to enter the **correct password** to open the **Royal Archives**.  

- If the password is `"RustKing"` → `🔒 Access granted! Welcome, scholar!`  
- If the password is wrong, print `❌ Incorrect password. Try again!`  
- If the user fails **three times**, print `🛋️ The gate is permanently locked!`  

**Challenge:** Use a `loop` with **break** to allow three attempts.  

---  

## 🎯 **Quest 7: The Archer's Training** 🏹  
You must shoot **10 arrows** at a target!  

- If the arrow hits `≥ 8`, print `🌟 Perfect shot!`  
- If the arrow hits `5-7`, print `🏹 Decent aim!`  
- If the arrow hits `< 5`, print `❌ You missed the target!`  

**Challenge:** Use a `for` loop to simulate **10 shots** with a **random hit score (1-10)**.  

---  

## ⚖️ **Quest 8: The Judge’s Dilemma** ⚖️  
A **wise judge** listens to **your case** and decides your **fate**:  

- If your **crime level (1-100) < 20**, print `⚖️ Innocent! You are free to go!`  
- If your **crime level is between 20-60**, print `⌛ You receive a fair punishment!`  
- If your **crime level is > 60**, print `🛢️ Sentenced to the dungeons!`  

**Challenge:** Use `if/else` to determine the verdict.  

---  

## 🚀 **Final Quest: The Hero's Choice** 🛡️  
At the end of your journey, you must **choose your destiny**:  

- **Enter the Kingdom** → `👑 You become the new ruler!`  
- **Go on another adventure** → `⚔️ Your legend continues!`  
- **Retire in peace** → `🌿 You live happily ever after!`  

**Challenge:** Use `match` to process the **player’s choice** (`1, 2, or 3`).  

---  

## 🛡️ Your Task  
- Solve **each quest** using Rust.  
- Save your solutions in `exercise.rs`.  
- Test your solutions and **improve your Rust skills!**  

🛡️ **Brave adventurer, your legend begins NOW!** 🚀🦀

