<script type="ts">
  import { onMount } from "svelte";

  let posts = [];

  onMount(async () => {
    const resp = await fetch("http://localhost:3939/timeline");
    posts = (await resp.json()).data;
  });
</script>

<h1>minichotan</h1>

<ul>
  {#each posts as post}
    <li>{post.text}</li>
  {/each}
</ul>
