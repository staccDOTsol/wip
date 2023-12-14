# Set up
 - Create your .env file from the example.env file, using your private key in base58 format
 - `npm i`

# To Run
    ts-node src/main.ts

# what?
    it plays svbonk.com for you. This is set up to run as an automated decentralized task on koii.network, however you can use these source files if you know what you are doing.
    it
    1. claims all your previous matches 
    2. uses SOL, swaps into bonk if you don't have enough
    3. buys an svbonk.com key every 5seconds
    4. if you don't like 3., replace this line of main if (true){//seconds <= 10){ with this if (seconds <= 10){
