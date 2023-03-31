Ractive.components.TicTacToeBoard = Ractive.extend({

    css:`
    
        .board{
            fill:#741b34;
        }

        .square.black{
            fill:#000000;
        }

        .square.Empty{
            fill:#aaaaaa;
        }

        .square.PlayerOne{
            fill:#ffffff;
            stroke:#000000;
            stroke-width:3;
        }

        .square.PlayerTwo{
            fill:#111111;
            stroke:#000000;
            stroke-width:3;
        }

        .Empty.highlighted:hover{
            fill:rgba(240,240,240,0.8);
        }

    `,


    template:`

    <h5 class="title is-5 is-info">
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
                Running {{iterations}} MCTS playouts
            {{/if}}
        {{/if}}
    </h5>


        <svg width="310" height="310" class="default">
            <g class="board">

                {{#each [0,1,2] as j}}
                    {{#each [0,1,2] as i}}
                        <g transform="translate({{i*(300/3)}},{{j*300/3}})">
                            {{#if (game.player()=="PlayerOne") & (game.result()=="InProgress")}}
                                <rect
                                    on-click="@this.fire('tictactoe_move', game, i,j)"
                                    x="5"  y="5" rx="3" ry="3" width="{{270/3}}"  height="{{270/3}}"
                                    class="square   highlighted {{game.cell(i,j)}}" >
                                </rect>
                            {{else}}
                                <rect
                                    x="5"  y="5" rx="3" ry="3" width="{{270/3}}"  height="{{270/3}}"
                                    class="square {{game.cell(i,j)}}  " >
                                </rect>
                            {{/if}}
                        </g>
                    {{/each}}
                {{/each}}

            </g>
        </svg>
<hr> 

        <pre>{{notes}}</pre>

    ` ,


    on: {
        'tictactoe_move'(context, game, i,j) {
            if (game.player() != "PlayerOne") {
                return;
            }
            if (game.cell(i,j) !="Empty"){
                return;
            }
            console.log('apply',i,j)

            let result = game.apply(i,j);

            console.log("result",result)
            this.update();
      
            if(game.result()!="InProgress")
                return


            let T = this;
            function inner() {
                console.log('inner')
                start = Date.now();
                let N = T.get('iterations');
                //let N = 1000;

                console.log('iterations',N)
                let suggested = game.suggest_move(N);

                console.log('suggested is',suggested.i, suggested.j)

                duration = (Date.now() - start) / 1000;

                T.set('notes', `Ran ${N} playouts in ${duration} seconds`);

                result = game.apply(suggested.i,suggested.j);
                console.log(result);

                T.update()
            }
            
            // allow paint to happen
            setTimeout(inner, 10);
       }
    }
});