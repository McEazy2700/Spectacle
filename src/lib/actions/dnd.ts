export function draggable<T>(node: HTMLElement, data: T) {
  let state = data
  node.draggable = true
  node.style.cursor = "grab"

  function handleDragStart(e: DragEvent) {
    e.dataTransfer?.setData("text/plain", JSON.stringify(data))
  }

  node.addEventListener("dragstart", handleDragStart)

  return {
    update(data: T) {
      state = data
    },
    destroy() {
      node.removeEventListener("dragstart", handleDragStart)
    }
  }
}

interface DropOptions<T> {
  dropEffect?: "move" | "copy" | "link" | "none"
  dragOverClass?: string
  onDrop?: (e: DragEvent, data: T) => void
}

export function dropzone<T>(node: HTMLElement, options?: DropOptions<T>) {
  let state: DropOptions<T> = {
    dropEffect: "move",
    ...options
  }

  function handleDragEnter(e: DragEvent) {
    (state.dragOverClass ?? "").split(" ").forEach(cls => {
      (e.target as HTMLElement).classList.add(cls)
    })
  }

  function handleDragLeave(e: DragEvent) {
    (state.dragOverClass ?? "").split(" ").forEach(cls => {
      (e.target as HTMLElement).classList.remove(cls)
    })
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault()
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = state.dropEffect ?? "move"
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault()
    const data = e.dataTransfer?.getData("text/plain");
    (state.dragOverClass ?? "").split(" ").forEach(cls => {
      (e.target as HTMLElement).classList.remove(cls)
    })
    state.onDrop && state.onDrop(e, data as T)
  }

  node.addEventListener("dragenter", handleDragEnter)
  node.addEventListener("dragleave", handleDragLeave)
  node.addEventListener("dragover", handleDragOver)
  node.addEventListener("drop", handleDrop)

  return {
    update(options: DropOptions<T>) {
      state = { ...state, ...options }
    },
    destroy() {
      node.removeEventListener("dragenter", handleDragEnter)
      node.removeEventListener("dragleave", handleDragLeave)
      node.removeEventListener("dragover", handleDragOver)
      node.removeEventListener("drop", handleDrop)
    }
  }
}
