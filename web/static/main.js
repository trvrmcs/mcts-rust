const MainScreen = Ractive.extend({
  css: `
  @media screen and (max-width: 768px) {
  .section {
    padding:0;
  }
}
  `,
  template: `
    
    
    <section class="section">
        <div class="container is-fluid">
        <h1 class="title">Monte Carlo Tree Search algorithm, Rust/WASM implementation</h1>
            
            <br>

            {{#if game}}
            <div class="has-text-centered">
 
            
                
                    <Connect4Board game={{game}} />
                
                
            </div>
            {{/if}}
             
           
        </div>
    </section>
    
  `,


});