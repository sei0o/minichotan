<script type="ts">
  import { onMount } from "svelte";

  let posts = [];

  onMount(async () => {
    const resp = await fetch("http://localhost:3939/timeline");
    posts = (await resp.json()).body;
  });

  function process(text: string): string {
    return text.replace(/https:\/\/t.co\/\w+(\s|$)/g, "");
  }
</script>

<h1>minichotan</h1>

<nav>
  <a href="/about">about</a>
</nav>

<main>
  {#each posts as post}
    <div class="post">{process(post.text)}</div>
  {/each}
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
