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

<ul>
  {#each posts as post}
    <li class="post">{process(post.text)}</li>
  {/each}
</ul>

<style>
.post {
  white-space: pre-line;
}
</style>
