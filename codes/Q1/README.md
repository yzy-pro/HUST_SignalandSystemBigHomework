# Q1: 频谱分析与频率偏差估计

## 项目结构

```
Q1/
├── Cargo.toml                   # Rust 项目配置文件
├── main.rs                      # 主程序入口
├── audio_reader.rs              # 模块 1: 音频文件读取
├── fft_processor.rs             # 模块 2: FFT 计算
├── spectrum_visualizer.rs       # 模块 3: 频谱可视化
├── frequency_estimator.rs       # 模块 4: 频率偏差估计
└── README.md                    # 本文件
```

## 功能说明

### 1. audio_reader.rs - 音频文件读取模块
- 使用 `hound` 库读取 WAV 文件
- 提取采样数据、采样率和样本数
- 支持多种音频格式（整数/浮点）
- 提供单声道转换功能
- 支持保存音频文件

**关键函数:**
- `AudioData::from_wav()`: 读取 WAV 文件
- `to_mono()`: 转换为单声道
- `save_wav()`: 保存音频文件

### 2. fft_processor.rs - FFT 计算模块
- 使用 `rustfft` 库进行快速傅里叶变换
- 计算频谱的幅度和相位
- 支持逆 FFT (IFFT)
- 提供频谱搬移功能
- 支持窗函数（Hanning、Hamming）

**关键函数:**
- `FftResult::compute()`: 计算 FFT
- `FftResult::ifft()`: 逆 FFT
- `get_single_sided()`: 获取单边频谱
- `circshift()`: 循环移位
- `frequency_shift_and_add()`: 频域搬移与相加

### 3. spectrum_visualizer.rs - 频谱可视化模块
- 使用 `plotters` 库绘制图形
- 支持线性和 dB 刻度
- 可绘制时域波形
- 支持多频谱对比

**关键函数:**
- `plot_spectrum()`: 绘制频谱图
- `plot_spectrum_db()`: 绘制 dB 刻度频谱
- `plot_waveform()`: 绘制时域波形
- `plot_spectrum_comparison()`: 绘制对比图

### 4. frequency_estimator.rs - 频率偏差估计模块
- 在频谱中搜索峰值
- 使用抛物线插值精确估计频率
- 支持多峰值检测
- 计算能量分布
- 分析频率关系

**关键函数:**
- `estimate_frequency_offset()`: 估计频率偏差
- `refined_frequency_estimate()`: 精确频率估计
- `find_multiple_peaks()`: 寻找多个峰值
- `compute_energy_distribution()`: 计算能量分布

## 编译和运行

### 前置要求
- Rust 工具链（推荐使用 rustup 安装）
- Cargo 包管理器

### 编译项目
```bash
cd codes/Q1
cargo build --release
```

### 运行程序
```bash
cargo run --release
```

### 运行测试
```bash
cargo test
```

## 输出文件

程序运行后会在 `output/` 目录下生成以下文件：

1. **Q1_spectrum_full.png** - 全频段频谱图
2. **Q1_spectrum_lowfreq.png** - 低频段频谱图 (0-10 kHz)
3. **Q1_spectrum_db.png** - dB 刻度频谱图
4. **Q1_waveform.png** - 时域波形图
5. **Q1_results.txt** - 分析结果文本文件

## 依赖库

- **hound**: WAV 文件读写
- **rustfft**: 快速傅里叶变换
- **plotters**: 图形绘制

## 理论基础

### FFT 频率分辨率
```
Δf = f_s / N
```
其中 f_s 是采样率，N 是采样点数。

### 频率与索引的对应关系
```
f_k = k · f_s / N,  k = 0, 1, ..., N-1
```

### 抛物线插值公式
用于精确估计峰值频率位置：
```
δ = 0.5 · (y₁ - y₃) / (y₁ - 2y₂ + y₃)
f_refined = f_peak + δ · Δf
```

## 注意事项

1. **音频文件路径**: 确保 `project.wav` 文件路径正确
2. **输出目录**: 程序会自动创建 `output` 目录
3. **内存使用**: 大文件可能需要较多内存
4. **采样率要求**: 确保采样率满足奈奎斯特采样定理

## 扩展功能

可以通过修改主程序添加以下功能：
- 不同窗函数的比较
- 频谱泄漏分析
- 信噪比估计
- 频谱峰值跟踪
