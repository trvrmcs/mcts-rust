
Ractive.components.CorridorBoard = Ractive.extend({
    data(){
        return {'selected':'nothing'}
    },
    css:`
        .board{
            fill:#741b34;
        }
        
        .square.black{
            fill:#000000;
        }
        
        .highlighted{
            fill:rgba(240,240,0,0.15);
        }
        .highlighted:hover{
            fill:rgba(240,240,0,0.8);
        }
        .wall{
            fill: #eedab6;
        }
        .wallslot{
            fill: #000000;
        }.
        
        .piece{
            
        }
        .piece.selected{
            stroke:yellow;
            stroke-width:4;
        }
    `,    
 

    applyCommand(command){
        console.log("here we go");
        console.log("command is",command);
        // if (game.player != "PlayerOne") {
        //     return;
        // }
        game=this.get('game')
        let result = game.apply(command);
        this.set('selected','nothing');
        console.log("result",result)

        this.update();
        if(game.result!="InProgress")
            return

    },

    on: {
        'hwall'(context, game, i, j){
            console.log('hwall',i,j);
            this.applyCommand(game.hwall_command(i,j));
        },
        'vwall'(context, game, i, j){
            console.log('vwall',i,j);
            this.applyCommand( game.vwall_command(i,j));
        },
        'move'(context, game, i,j){
            console.log('move',i,j);
            this.applyCommand(game.move_to_command(i,j));
        },
        'hop'(context,game, d1, d2){
            console.log('hop',d1,d2);
            console.log("not yet implemented")
        }
    },
    template:`
 

    <svg width="650" height="500" class=" default">

        <g class="board" width="650" with="500" transform="scale(1,-1) translate(0,-500)">
        
            <rect width="650" height="500" class="border"></rect>
           
            <g class="player_two_walls" transform="translate(0,15)">
                
                {{#each [0,1,2,3,4,5,6,7,8,9] as i}}
                    <rect 
                        x="5" y="{{i*50}}" width="90" height="8"   rx="2" ry="2" 
                        class="{{i<game.player2.walls?'wall':'wallslot'}}"
                    >
                    </rect>
                {{/each}}
            </g>

            <g class="player_one_walls" transform="translate(550,15)">
                        
                {{#each [0,1,2,3,4,5,6,7,8,9] as i}}
                    {{#if game.player=='PlayerOne' && game.result=='InProgress' && game.player1.walls}}
                        <rect 
                            on-click="@this.set('selected',selected=='hwall'?'vwall':'hwall'   )"
                            x="5" y="{{i*50}}" width="90" height="8"   rx="2" ry="2" 
                            class="{{i<game.player1.walls?'wall':'wallslot'}}"
                        ></rect>
                    {{else}}
                        <rect 
                            x="5" y="{{i*50}}" width="90" height="8"   rx="2" ry="2" 
                            class="{{i<game.player1.walls?'wall':'wallslot'}}"
                        ></rect>
                    {{/if}}
                
                {{/each}}
            </g>
            <g transform="translate(100,20)" class="squares ">
                {{#each [0,1,2,3,4,5,6,7,8] as j}}
                    {{#each [0,1,2,3,4,5,6,7,8] as i}}
                        <rect 
                            x="{{5+(i*50)}}" y="{{( 5+(j*50))}}" rx="3" ry="3" width="40" height="40" 
                            class="square black "
                        />

                        {{#if selected=='piece'  && game.can_move_to(i,j)}}      
                            <rect 
                                x="{{5+(i*50)}}" 
                                y="{{( 5+(j*50))}}" 
                                rx="3" ry="3" 
                                width="40" height="40" 
                                class="square highlighted "
                                on-click="@this.fire('move',game, i,j)"
                            />
                        {{/if}}
                    {{/each}}
                {{/each}}
 

                <g class="piece player_one" transform="translate({{50*game.player1.location.i}},{{50*game.player1.location.j}})">
                    <circle cx="25" cy="25" r="15" stroke="black" stroke-width="1" fill="red" on-click="@this.set('selected','piece')" />
                </g>
                <g class="piece player_two" transform="translate({{50*game.player2.location.i}},{{50*game.player2.location.j}})">
                    <circle cx="25" cy="25" r="15" stroke="black" stroke-width="1" fill="blue" on-click="@this.set('selected','piece')" />
                </g>

                <g  class="wall_slots">
                    {{#each [0,1,2,3,4,5,6,7] as i}}
                        {{#each [0,1,2,3,4,5,6,7] as j}}
                            {{#if selected=='hwall' && game.can_place_hwall(i,j)}}
                                <rect
                                    on-click="@this.fire('hwall',game,i,j)"
                                x="{{5+ i*50}}" y="{{46+j*50}}" width="90" height="8"   rx="2" ry="2" class="highlighted" 
                                />
                            {{/if}}
                            {{#if selected=='vwall' && game.can_place_vwall(i,j)}}
                                <rect
                                    on-click="@this.fire('vwall',game,i,j)"
                                    x="{{46 + i*50}}" y="{{5+j*50}}" width="8" height="90"   rx="2" ry="2" class="highlighted" 
                                />
                            {{/if}}
                            {{#if game.hwall_at(i,j)}}
                                <rect x="{{5 + i*50}}" y="{{46+j*50}}" width="90" height="8"   rx="2" ry="2" class="wall" />
                            {{/if}}
                            {{#if game.vwall_at(i,j)}}
                                <rect x="{{46 + i*50}}" y="{{5+j*50}}" width="8" height="90"   rx="2" ry="2" class="wall" />
                            {{/if}}
                        {{/each}}
                    {{/each}}
                
                </g>

            </g>


        </svg>
 
        <pre  >
            DEBUG

            Selected: {{selected}}
            {{#if selected=='piece' }}
            YEP, piece selected
            {{/if}}

            Result: {{game.result}}
            
            Player 1 

                walls: {{game.player1.walls}}
                location: ({{game.player1.location.i}}, {{game.player1.location.j}})

            Player 2:
                walls: {{game.player2.walls}}
                location: ({{game.player2.location.i}}, {{game.player2.location.j}})

            current: {{game.player}}


            
            
            
        </pre>

    `
});