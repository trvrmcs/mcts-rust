/*
    I want a mechanism for highlighting legal moves.
    
    This means 2 things: SENDING the legal moves with the board state.
    Painting them in response to mouse activity.
    
    Maybe we have a clickable overlay when it's your turn?
    
    
*/
Ractive.components.Connect4Board = Ractive.extend({
    data: { game: null },
    css: `
        .board{
            fill:#0000ff;
        }
        
        .square.black{
            fill:#000000;
        }
        .slot.PlayerOne{
            fill:red;
        }
        .slot.PlayerTwo{
            fill:yellow;
        }
        
        .target.PlayerOne:hover{
            fill:red;
        }
         
    `,

    template: `  <button class="button" on-click="@this.fire('new_game',game)">New Game</button>
        <h4 class="title is-4"> 
            {{#if game.result()=="PlayerOne"}}
                You win!
            {{/if}}
            {{#if game.result()=="PlayerTwo"}}
                You lose!
            {{/if}}
            {{#if game.result()=="Draw"}}
                Draw
            {{/if}}
            {{#if game.result()=="InProgress"}}
                {{#if game.player()=="PlayerOne"}}
                    Your turn
                {{else}}
                    Thinking...
                {{/if}}
            {{/if}}
        </h4>
    
        <svg width="350" height="350" class="default">
        
            {{#if (game.result()=="InProgress")}}
     
                <rect class="board drop" width="350" height="40">
                
                </rect>
                
                {{#each [0,1,2,3,4,5,6] as i}}
                    <g transform="translate({{i*50}},0)">
                      <rect x="5" y="5" width="40" height="30" rx="5" 
                        class="target {{game.player()}}"
                        on-click="@this.fire("connect4_move",game, i)"
                      />
     
                    </g>
                {{/each}}
            {{/if}}
            
            <rect class="board" y="50" width="350" height="300">
            </rect>
            
            
            {{#each [0,1,2,3,4,5] as j}}
                {{#each [0,1,2,3,4,5,6] as i}}
                      <g transform="translate({{i*50}},{{300-j*50}})">
                      <circle cx="25" cy="25" r="20" stroke="black" stroke-width="3" class="slot {{game.cell(i,j)}}"
                        
                      />
                    </g>
                {{/each}}
            {{/each}}
        </svg>
        <pre>
        {{notes}}
        </pre>
     
    `,

    on: {
        'new_game'(context, game) {
            game.reset();
            this.update();
        },
        'connect4_move'(context, game, column) {
            if (game.player() != "PlayerOne") {
                console.log("Not your turn");
                return;
            }

            let result = game.apply(column);

            let T = this;
            function inner() {

                start = Date.now();
                let N = 100000;
                let suggested = game.suggest_move(N);
                duration = (Date.now() - start) / 1000;

                T.set('notes', `Ran ${N} playouts in ${duration} seconds`);

                result = game.apply(suggested);

                T.update()


            }
            this.update();
            // allow paint to happen
            setTimeout(inner, 10);



        }
    }
});