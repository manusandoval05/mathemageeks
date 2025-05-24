<script lang="ts">
    export let node;
    export let level = 0;
    
    const nodeIsArray = Array.isArray(node);

    let expressionArray = [];
    if(nodeIsArray){
        expressionArray = node.map((relationship: any) => ({
            name: relationship.name,
            expression: relationship.expression,
            equivalences: relationship.es_equivalente_a,
            generality: relationship.es_caso_particular_de,
            resolutions: relationship.se_resuelve_con,
            applications: relationship.tiene_aplicacion_en,
            instances: relationship.es_instancia_de,
        }));
    }
</script>

{#if nodeIsArray}
    {#each expressionArray as expression}
    <div>
        <h1>{expression.name}</h1>
        <div>
          {expression.expression}
        </div>
        <h2>Demostración</h2>
        <div>
            <button>Demostración</button>
        </div>
        <h2>Encontrado en</h2>
        <ul>
          {#each expression.instances as instance}
            <li>{instance}</li>
          {/each}
        </ul>
        {#each expression.equivalences as equivalencia}
          <div>{equivalencia}</div>
        {/each}
        <h2>Es un caso particular de:</h2>
        {#each expression.generality as generality}
          <div>{generality}</div>
        {/each}
        <h2>Se resuelve con</h2>
        <ul>
          {#each expression.resolutions as resolution}
            <li>{resolution}</li>
          {/each}
        </ul>
        <h2>Aplicaciones importantes</h2>
        <ul>
          {#each expression.applications as application}
            <li>{application}</li>
          {/each}
        </ul>
      </div>
    {/each}
{:else}

{/if}