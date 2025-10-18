<template>
  <div
    class="fixed inset-0 z-50 flex items-end justify-center backdrop-container"
    @click="handleBackdropClick"
  >
    <!-- Modal Content -->
    <div
      ref="modalRef"
      @click.stop
      class="relative w-full max-w-2xl rounded-t-3xl shadow-2xl modal-content flex"
      :class="{
        'animate-slide-up': isAnimating,
        'transition-none': isDragging,
        'transition-transform duration-300 ease-out': !isDragging,
      }"
      :style="modalStyle"
    >
      <!-- Visible Content Wrapper -->
      <div
        class="relative bg-white rounded-t-3xl border-t border-secondary pb-safe z-20 overflow-hidden flex-1"
      >
        <!-- Drag Handle -->
        <div
          @touchstart="handleTouchStart"
          @touchmove="handleTouchMove"
          @touchend="handleTouchEnd"
          @mousedown="handleMouseDown"
          class="flex justify-center pt-3 pb-2 cursor-grab active:cursor-grabbing"
        >
          <div class="w-12 h-1.5 bg-text-secondary/30 rounded-full"></div>
        </div>

        <!-- Header -->
        <div
          class="sticky top-0 z-20 bg-white border-b border-secondary px-6 pb-4"
        >
          <div class="flex items-center justify-between mb-2">
            <h2 class="text-2xl font-bold text-text-primary">{{ title }}</h2>
            <button
              @click="handleClose"
              class="p-2 rounded-full hover:bg-secondary transition-colors"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 text-text-secondary"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>
          <slot name="header-content"></slot>
        </div>

        <!-- Content -->
        <div
          class="overflow-y-auto p-6 pt-4 flex-1"
          :style="{ maxHeight: `calc(${height} - ${headerHeight}px)` }"
        >
          <slot></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";

interface Props {
  title: string;
  height?: string;
  headerHeight?: number;
  closeOnBackdrop?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  height: "75vh",
  headerHeight: 180,
  closeOnBackdrop: true,
});

const emit = defineEmits<{
  close: [];
}>();

// Drag functionality
const modalRef = ref<HTMLElement | null>(null);
const isDragging = ref(false);
const isAnimating = ref(true);
const startY = ref(0);
const currentY = ref(0);
const translateY = ref(0);
const lastY = ref(0);
const lastTime = ref(0);
const velocity = ref(0);

const modalStyle = computed(() => {
  const transform = `translateY(${translateY.value}px)`;
  return {
    height: props.height,
    maxHeight: props.height,
    transform,
  };
});

const handleTouchStart = (e: TouchEvent) => {
  isDragging.value = true;
  isAnimating.value = false;
  startY.value = e.touches[0].clientY;
  currentY.value = translateY.value;
  lastY.value = e.touches[0].clientY;
  lastTime.value = Date.now();
  velocity.value = 0;
};

const handleTouchMove = (e: TouchEvent) => {
  if (!isDragging.value) return;

  e.preventDefault();

  const now = Date.now();
  const currentTouchY = e.touches[0].clientY;
  const deltaY = currentTouchY - startY.value;

  const timeDelta = now - lastTime.value;
  if (timeDelta > 0) {
    velocity.value = (currentTouchY - lastY.value) / timeDelta;
  }

  lastY.value = currentTouchY;
  lastTime.value = now;

  if (deltaY > 0) {
    translateY.value = currentY.value + deltaY;
  } else if (deltaY < 0) {
    translateY.value = currentY.value + deltaY * 0.3;
  }
};

const handleTouchEnd = () => {
  if (!isDragging.value) return;

  isDragging.value = false;

  const modalHeight = modalRef.value?.offsetHeight || window.innerHeight * 0.75;
  const threshold = modalHeight * 0.25;

  const isSwipeDown = velocity.value > 0.5;

  if (translateY.value > threshold || isSwipeDown) {
    const windowHeight = window.innerHeight;
    translateY.value = windowHeight;
    setTimeout(() => {
      handleClose();
    }, 300);
  } else {
    translateY.value = 0;
  }
};

const handleMouseDown = (e: MouseEvent) => {
  isDragging.value = true;
  isAnimating.value = false;
  startY.value = e.clientY;
  currentY.value = translateY.value;
  lastY.value = e.clientY;
  lastTime.value = Date.now();
  velocity.value = 0;

  const handleMouseMove = (e: MouseEvent) => {
    if (!isDragging.value) return;

    const now = Date.now();
    const currentMouseY = e.clientY;
    const deltaY = currentMouseY - startY.value;

    const timeDelta = now - lastTime.value;
    if (timeDelta > 0) {
      velocity.value = (currentMouseY - lastY.value) / timeDelta;
    }

    lastY.value = currentMouseY;
    lastTime.value = now;

    if (deltaY > 0) {
      translateY.value = currentY.value + deltaY;
    } else if (deltaY < 0) {
      translateY.value = currentY.value + deltaY * 0.3;
    }
  };

  const handleMouseUp = () => {
    if (!isDragging.value) return;

    isDragging.value = false;

    const modalHeight =
      modalRef.value?.offsetHeight || window.innerHeight * 0.75;
    const threshold = modalHeight * 0.25;

    const isSwipeDown = velocity.value > 0.5;

    if (translateY.value > threshold || isSwipeDown) {
      const windowHeight = window.innerHeight;
      translateY.value = windowHeight;
      setTimeout(() => {
        handleClose();
      }, 300);
    } else {
      translateY.value = 0;
    }

    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  };

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
};

const handleBackdropClick = () => {
  if (props.closeOnBackdrop) {
    handleClose();
  }
};

const handleClose = () => {
  emit("close");
};

onMounted(() => {
  setTimeout(() => {
    isAnimating.value = false;
  }, 300);
});
</script>

<style scoped>
.backdrop-container::before {
  content: "";
  position: absolute;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  z-index: -1;
}

.modal-content::after {
  content: "";
  position: absolute;
  top: 99%;
  left: 0;
  right: 0;
  height: 100vh;
  background-color: white;
  z-index: 10;
}

@keyframes slide-up {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.animate-slide-up {
  animation: slide-up 0.3s ease-out;
}
</style>
