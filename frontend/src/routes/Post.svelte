<script type='ts'>
  export let post;

  let opened = false;
  let processedText;
  $: processedText = post.text.replace(/https:\/\/t.co\/\w+(\s|$)/g, "");

  function toggle() {
    opened = !opened;
  }
</script>

<div class="post" on:click={toggle}>
  <span class="text">{processedText}</span>
  <div class="detail" class:open={opened}>
    <span class="debug">{JSON.stringify(post)}</span>
  </div>
</div>

<style>
  .post {
    white-space: pre-line;
    width: 100%;
    border-top: 1px solid #ccc;
    padding: 0.2rem 0;
    min-height: 1rem;
  }

  .debug {
    font-size: 0.6rem;
    color: #666;
  }

  .detail {
    display: none;
  }

  .detail.open {
    display: block;
  }
</style>
