<script type="ts">
  import { onMount } from "svelte";
  import Post from './Post.svelte';

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

</script>

<header>
  <nav>
    <ul>
      <li>minichotan</li>
      {#each userIds as id}
        <li><a href="#" on:click={setUser(id)}>{id}</a></li>
      {/each}
      <li><a href="/about">about</a></li>
    </ul>
  </nav>
</header>

<main>
  {#if currentUserId}
    {#each posts[currentUserId] as post}
      <Post {post} />
    {/each}
  {/if}
</main>

<style>
body {
  margin: 0;
  padding: 0;
  border: none;
}

header {
}

nav {
  margin-bottom: 0.5rem;
  font-size: 0.8rem;
  color: #333;
}

nav ul {
  display: inline;
  margin: 0;
  padding: 0;
}

nav li {
  display: inline;
}

nav a {
  text-decoration: none;
}

nav a:hover {
  text-decoration: underline;
}

main {
  padding: 0;
}

</style>
