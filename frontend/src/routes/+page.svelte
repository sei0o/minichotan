<script type="ts">
  import { onMount } from "svelte";

  let posts = {};
  let userIds = [];
  let currentUserId = null;

  onMount(async () => {
    const acctResp = await fetch("http://localhost:3939/accounts");
    userIds = (await acctResp.json()).user_ids;
  });

  // api_paramsを含まないと肩が一意に定まらない
  async function setUser(userId: string) {
    const resp = await fetch(`http://localhost:3939/timeline?user_id=${userId}&api_params={}`);
    posts[userId] = (await resp.json()).body;

    currentUserId = userId;
  }

  function process(text: string): string {
    return text.replace(/https:\/\/t.co\/\w+(\s|$)/g, "");
  }
</script>

<h1>minichotan</h1>

<nav>
  <a href="/about">about</a>
  {#each userIds as id}
    <a href="#" on:click={setUser(id)}>{id}</a>
  {/each}
</nav>

<main>
  {#if currentUserId}
    {#each posts[currentUserId] as post}
      <div class="post">{process(post.text)}</div>
    {/each}
  {/if}
</main>

<style>
body {
  margin: 0;
  padding: 0;
  border: none;
}

nav {
  margin-bottom: 0.5rem;
}

main {
  padding: 0;
}

.post {
  white-space: pre-line;
  width: 100%;
  border-top: 1px solid #ccc;
  padding: 0.2rem 0;
  min-height: 1rem;
}
</style>
