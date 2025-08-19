<template>
    <div class="relative w-full h-full">

        <slot class="absolute w-full h-full bg-transparent" />
        <div class="absolute w-full h-full overflow-hidden" :data-url="backgroundImage">
            <canvas class="absolute inset-0 w-full h-full" ref="canvasRef"></canvas>
        </div>
    </div>

</template>

<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from 'vue';

// 定义组件Props类型
const props = defineProps<{
    backgroundImage: string;
    brightness?: number; // 建议范围：0-200（百分比）
    saturation?: number; // 建议范围：0-200（百分比）
    contrast?: number;   // 建议范围：0-200（百分比）
    vignetteIntensity?: number; // 暗角强度：0-100，默认50
    vignetteSize?: number; // 暗角大小：0-100，默认70
}>();

// 设置默认值
const defaultBrightness = 100;
const defaultSaturation = 100;
const defaultContrast = 100;
const defaultVignetteIntensity = 50; // 暗角强度默认值
const defaultVignetteSize = 70; // 暗角大小默认值

// Canvas元素引用
const canvasRef = ref<HTMLCanvasElement | null>(null);

// 动画相关变量
let animationId: number | null = null;
let backgroundImage: HTMLImageElement | null = null;
let position = { x: 0, y: 0 };
let rotation = 0;
let velocity = { x: 0, y: 0 };
let rotationSpeed = 0;

// 初始化背景图片
const initBackgroundImage = () => {
    if (backgroundImage) {
        backgroundImage.remove();
    }

    backgroundImage = new Image();
    backgroundImage.crossOrigin = 'anonymous';
    backgroundImage.src = props.backgroundImage;

    backgroundImage.onload = () => {
        if (canvasRef.value) {
            startAnimation();
        }
    };

    backgroundImage.onerror = (error) => {
        console.error('背景图片加载失败:', error);
    };
};

// 设置随机动画参数 - 调整为更缓慢的速度
const setRandomAnimationParameters = () => {
    // 初始位置范围
    position = {
        x: (Math.random() - 0.5) * 200,
        y: (Math.random() - 0.5) * 200
    };

    // 随机初始旋转角度
    rotation = Math.random() * 360;

    // 移动速度减慢 - 更平缓的移动
    velocity = {
        x: (Math.random() - 0.5) * 0.02,  // 减慢到原来的1/5
        y: (Math.random() - 0.5) * 0.02   // 减慢到原来的1/5
    };

    // 旋转速度减慢 - 更缓慢的旋转
    rotationSpeed = (Math.random() - 0.5) * 0.01;  // 减慢到原来的1/10
};

// 绘制暗角效果
const drawVignette = (ctx: CanvasRenderingContext2D, width: number, height: number) => {
    // 获取暗角参数，限制在有效范围内
    const intensity = Math.max(0, Math.min(100, props.vignetteIntensity ?? defaultVignetteIntensity)) / 100;
    const size = Math.max(0, Math.min(100, props.vignetteSize ?? defaultVignetteSize)) / 100;

    // 创建径向渐变，从中心到角落
    const gradient = ctx.createRadialGradient(
        width / 2, height / 2,  // 渐变中心
        width * 0.1 * size,     // 渐变开始半径（中心亮区）
        width / 2, height / 2,  // 渐变中心
        width * 0.8             // 渐变结束半径（覆盖整个画布）
    );

    // 渐变颜色设置 - 从完全透明到黑色，使用intensity控制暗度
    gradient.addColorStop(0, `rgba(0, 0, 0, 0)`);
    gradient.addColorStop(0.6 * size, `rgba(0, 0, 0, 0)`);
    gradient.addColorStop(1, `rgba(0, 0, 0, ${intensity})`);

    // 绘制渐变覆盖层作为暗角
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, width, height);
};

// 绘制背景到Canvas
const drawBackground = () => {
    if (!canvasRef.value || !backgroundImage) return;

    const canvas = canvasRef.value;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // 设置Canvas尺寸为窗口大小
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // 清除画布
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // 保存当前上下文状态
    ctx.save();

    // 计算放大倍数
    const scale = 0.8;
    const scaledWidth = backgroundImage.width * scale;
    const scaledHeight = backgroundImage.height * scale;

    // 将原点移动到画布中心
    ctx.translate(canvas.width / 2, canvas.height / 2);

    // 应用旋转
    ctx.rotate(rotation * Math.PI / 180);

    // 应用滤镜
    ctx.filter = `
      brightness(${props.brightness ?? defaultBrightness}%) 
      saturate(${props.saturation ?? defaultSaturation}%) 
      contrast(${props.contrast ?? defaultContrast}%) 
      blur(100px)
    `;

    // 绘制图像
    ctx.drawImage(
        backgroundImage,
        -scaledWidth / 2 + position.x,
        -scaledHeight / 2 + position.y,
        scaledWidth,
        scaledHeight
    );

    // 恢复上下文状态
    ctx.restore();

    // 绘制暗角效果（在所有内容之上）
    drawVignette(ctx, canvas.width, canvas.height);
};

// 更新动画状态
const updateAnimation = () => {
    if (!backgroundImage) return;

    // 更新位置（速度已减慢）
    position.x += velocity.x;
    position.y += velocity.y;

    // 更新旋转角度（速度已减慢）
    rotation += rotationSpeed;

    // 边界范围
    const boundary = 200;
    if (position.x > boundary || position.x < -boundary) {
        velocity.x = -velocity.x * 0.85;
    }

    if (position.y > boundary || position.y < -boundary) {
        velocity.y = -velocity.y * 0.85;
    }

    // 偶尔随机改变速度，变化更平缓
    if (Math.random() < 0.002) {  // 降低触发概率
        velocity.x += (Math.random() - 0.5) * 0.02;  // 更小的速度变化
        velocity.y += (Math.random() - 0.5) * 0.02;
    }

    // 重新绘制背景
    drawBackground();

    // 继续动画循环
    animationId = requestAnimationFrame(updateAnimation);
};

// 开始动画
const startAnimation = () => {
    if (animationId) {
        cancelAnimationFrame(animationId);
    }

    setRandomAnimationParameters();
    drawBackground();
    animationId = requestAnimationFrame(updateAnimation);
};

// 停止动画
const stopAnimation = () => {
    if (animationId) {
        cancelAnimationFrame(animationId);
        animationId = null;
    }
};

// 处理窗口大小变化
const handleWindowResize = () => {
    drawBackground();
};

// 组件挂载时初始化
onMounted(() => {
    window.addEventListener('resize', handleWindowResize);
    if (props.backgroundImage) {
        initBackgroundImage();
    }
});

// 监听Props变化
watch([
    () => props.backgroundImage,
    () => props.brightness,
    () => props.saturation,
    () => props.contrast,
    () => props.vignetteIntensity,
    () => props.vignetteSize
], () => {
    if (props.backgroundImage) {
        if (backgroundImage && backgroundImage.src !== props.backgroundImage) {
            stopAnimation();
            initBackgroundImage();
        } else {
            drawBackground();
        }
    }
});

// 组件卸载时清理
onUnmounted(() => {
    stopAnimation();
    window.removeEventListener('resize', handleWindowResize);
    backgroundImage = null;
});
</script>

<style scoped>
/* 确保canvas覆盖整个容器 */
:deep(canvas) {
    object-fit: cover;
}
</style>
