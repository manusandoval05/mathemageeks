<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { level } from "$lib/store";
  import katex from 'katex';
  import { tick } from 'svelte';
  import renderMathInElement from "https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/contrib/auto-render.mjs";
  import AsciiMathParser from "asciimath2tex";


  const parser = new AsciiMathParser();
  
  let expression = $state("");

  let renderedExpression = $state("");

  let expressionsHtmlList: string[] = $state([]);
  let expressionList: any = $state([]);

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

    const tex = parser.parse(expression);
    
    await tick();
    renderMathInElement(document.body, {
      delimiters: [
        {left: '$', right: '$', display: true},
      ],
      // customised options
      // • auto-render specific keys, e.g.:
      // • rendering keys, e.g.:
      throwOnError : false
    });
    const intuition = await getMathEquality([tex]);
    console.log(intuition.response);

    const relationship = {
      name: intuition.response.name,
      expression: intuition.response.expression,
      equivalences: intuition.response.es_equivalente_a,
      generality: intuition.response.es_caso_particular_de,
      resolutions: intuition.response.se_resuelve_con,
      applications: intuition.response.tiene_aplicacion_en,
      instances: intuition.response.es_instancia_de,
    }

    expressionList = [...expressionList, intuition.response];

    await tick();

    renderMathInElement(document.body, {
      // customised options
      // • auto-render specific keys, e.g.:
      delimiters: [
        {left: '$', right: '$', display: true}
      ],
      // • rendering keys, e.g.:
      throwOnError : false
    });
  }
  function newConversation(){
    expressionList = [];
    expressionsHtmlList = [];
    expression = "";
    renderedExpression = "";
  }
  
  async function getMathEquality(expressions: string[]): Promise<any> {
    console.log("Calling APi");
    const res = await fetch('http://127.0.0.1:8000/get-expression', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ expressions })
    });
    const data = await res.json();

    return data;
  }
  async function refreshExpression(){
    const tex = parser.parse(expression);
    const html = katex.renderToString(tex, {
      throwOnError: false,
      displayMode: true
    });
    renderedExpression = html;
  }
</script>

<main class="container">
  <button class="new-conversation-button" onclick={newConversation}>Nuevos cálculos</button>
  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/logo.png" class="logo vite" alt="Vite Logo" />
    </a>
  </div>
  <h1>Mathemageeks</h1>
  {@html renderedExpression}
  <form class="row" onsubmit={greet}>
    <input id="greet-input" onkeypress={refreshExpression} placeholder="Escribe una expresión lógica o matemática..." bind:value={expression} />
    <button type="submit">M</button>
  </form>
  <div>
    <ol class="flex items-center gap-4">
    <li><a class="opacity-60 hover:underline" href="#">Blog</a></li>
    <li class="opacity-50" aria-hidden>&rsaquo;</li>
    <li><a class="opacity-60 hover:underline" href="#">Category</a></li>
    <li class="opacity-50" aria-hidden>&rsaquo;</li>
    <li>Article</li>
  </ol>
  </div>
  {#each expressionList as customExpression}
    <div>
      <h1>{customExpression.name}</h1>
      <div>
        {customExpression.expression}
      </div>
      <div>
        <button>Demostración</button>
      </div>
      <h2>Encontrado en</h2>
      <ul>
        {#each customExpression.instances as instance}
          <li>{instance}</li>
        {/each}
      </ul>
      {#each customExpression.equivalences as equivalencia}
        <div>{equivalencia}</div>
      {/each}
      <h2>Es un caso particular de:</h2>
      {#each customExpression.generality as generality}
        <div>{generality}</div>
      {/each}
      <h2>Se resuelve con</h2>
      <ul>
        {#each customExpression.resolutions as resolution}
          <li>{resolution}</li>
        {/each}
      </ul>
      <h2>Aplicaciones importantes</h2>
      <ul>
        {#each customExpression.applications as application}
          <li>{application}</li>
        {/each}
      </ul>
    </div>
  {/each}

  
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 10em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}
.new-conversation-button {
  width: 25%;
  max-width: 640px;
  align-self: center;
  
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
  width: 80%;
  max-width: 500px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
