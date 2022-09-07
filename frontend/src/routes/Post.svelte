<script type='ts'>
  export let post;
  export let users = {};

  let opened = false;
  let processedText;
  $: {
    processedText = post.text
      .replace(/https:\/\/t.co\/\w+(\s|$)/g, "")
      .replace(/^RT @([^:])+: /g, "");
    processedText = decodeURIComponent(processedText);
  }

  let author = users[post.author_id];

  function toggle() {
    opened = !opened;
  }
</script>

<div class="post" on:click={toggle}>
  <span class="text">{processedText}</span>
  <div class="detail" class:open={opened}>
    {#if author}
      <span class="author">@{author.username}</span>
    {/if}
    <span class="debug">{JSON.stringify(post)}</span>
  </div>
</div>

<style>
  .post {
    white-space: pre-line;
    width: 100%;
    border-top: 1px solid #ddd;
    padding: 0.2rem 0;
    min-height: 1rem;
  }

  .debug {
    display: block;
    font-size: 0.6rem;
    color: #666;
  }

  .detail {
    display: none;
  }

  .detail.open {
    display: block;
  }

  .author {
    font-size: 0.8rem;
  }
</style>
