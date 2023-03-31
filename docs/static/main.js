import init, { Connect4Game, TicTacToeGame } from "../pkg/mcts.js";


function create_game(game_type) {

  console.log("new game of type", game_type)
  let constructor = {
    'tictactoe': TicTacToeGame,
    'connect4': Connect4Game
  }[game_type];

  return new constructor();

}

const MainScreen = Ractive.extend({
  css: `
  @media screen and (max-width: 768px) {
  .section {
    padding:0;
  }
}
  `,
  on: {
    'new_game'(context, game_type) {
      var game = create_game(game_type)
      this.set({ 'game': game, 'gametype': game_type });

    }
  },
  template: `
    
  <section class="hero is-info">
    <div class="hero-body">
      <div class="container">
        <h3 class="title is-3">Monte Carlo Tree Search</h3>
        <h5 class="title is-5">Rust/WASM implementation</h5>
      </div>
    </div>
  </section>
 
  <section class="section">
    <div class="container">
      <div class="has-text-centered"><h7 class="title is-5">Iterations</h7></div>
      <div class="buttons has-addons is-centered">
        {{#each [100,1000,10000,100000] as it}}
          <button on-click="@this.set('iterations',it)" class="button is-small is-rounded {{iterations==it?'is-info':''}}">{{it}}</button>
        {{/each}}
        
      </div>

      <div class="has-text-centered"><h7 class="title is-5">Game</h7></div>
      <div class="buttons has-addons is-centered">
      
      <button class="button is-rounded {{gametype=='tictactoe'?'is-info':''}}" on-click="@this.fire('new_game','tictactoe')">
       TicTacToe
      </button>
      <button class="button is-rounded {{gametype=='connect4'?'is-info':''}}" on-click="@this.fire('new_game','connect4')">
       Connect4
      </button> 
      </div>
 
 


      <div class="has-text-centered">
         {{#if gametype=='connect4'}}
          <Connect4Board game={{game}} iterations={{iterations}}/>
        {{/if}}
        {{#if gametype=='tictactoe'}}
          <TicTacToeBoard game={{game}} iterations={{iterations}}/>
        {{/if}}
      </div>

    </div>

  </section>
     
  `,
});




init().then(() => {


  let game_type = 'tictactoe';
  let game = create_game(game_type);

  new MainScreen({ el: "#main", data: { game: game, gametype: game_type, iterations:1000 } });

});
