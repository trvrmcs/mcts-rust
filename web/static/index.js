import init, { Connect4Game } from "/pkg/mcts.js";


init().then(() => {

    console.log("HERE WE GO");

    var game = new Connect4Game();
    window.THE_GAME = game;

    new MainScreen({ el: "#main", data: { game: game } });


});
