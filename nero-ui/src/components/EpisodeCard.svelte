<script lang="ts">
  import { link } from "../pages/Router.svelte";
  import type { Episode } from "../types/episode";

  let { seriesId, episode }: { seriesId: string; episode: Episode } = $props();
</script>

<a
  class="grid grid-cols-[1fr_4fr_7fr] items-center gap-4 rounded-md p-1 duration-300
    hover:bg-gray-100 active:scale-95"
  href="/watch/{seriesId}/{episode.id}"
  use:link
>
  <span class="truncate text-center font-semibold">{episode.number}</span>
  <picture class="aspect-video min-w-[150px]">
    <img
      class="size-full rounded-lg object-cover"
      src={episode.thumbnailUrl}
      alt="Episode thumbnail"
    />
  </picture>
  <div class="min-w-0">
    <h3 class="truncate font-semibold">
      {episode.title ?? `Episode ${episode.number}`}
    </h3>
    <p class="line-clamp-3 text-sm text-gray-500">
      {episode.description}
    </p>
  </div>
</a>
