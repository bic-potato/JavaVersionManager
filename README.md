# JavaVersionManager

JavaVersionMANager(JVMAN)，一个基于 [Elipse Adoptium API](https://api.adoptium.net/) 使用 Rust 开发的轻量级 JDK 版本管理应用，支持 Windows 平台。

## 使用方法

### 下载和安装

`list remote` 命令可以列出支持的 JDK 主版本号，使用 `get remote {version}` 指令即可下载对应版本的最新版 JDK.例如：

```cmd
jvman list remote // 获取可用的主版本号

jvman get remote 11 // 下载 JDK 11 的最新构建版
```

### 加载本地jdk环境

`get local {Path}` 可以从路径中获取已安装的JDK的版本信息`注：目前能够完美支持的只有Adopt OpenJDK系列`. 例如:

```cmd
"C:/Program Files/Eclipse Foundation/jdk-16.0.2.7-hotspot"

//加载位于该位置的JDK
```

### 本地已安装的 JDK 版本的查看和使用

`list local` 命令可以查看当前已安装版本的版本列表， 使用`enable -i {implementor} -v {version}` 命令可以全局启用特定版本的JDK。例如：

```cmd
jvman list local // 列出本机的所有已下载的JDK
/*
将会呈现的输出类似于如下形式：
All available JDKs:
        "Eclipse Adoptium" jdk-11.0.14.1+1
        "Eclipse Foundation" 16.0.2+7
        "Eclipse Adoptium" jdk8u322-b06
*/
jvman enable -i "Eclipse Adoptium" -v jdk-11.0.14.1+1 // 启用版本为 jdk-11.0.14.1+1 的 JDK
```

**请注意**，由于当前版本尚未实现 UAC 提权~~（很可能以后也不会实现）~~，所以 `jvman enable` 命令会出现 `Enable FAILED, 客户端没有所需的特权。 (os error 1314)` 的错误报告，推荐搭配 [gsudo](https://github.com/gerardog/gsudo) 使用，安装后使用方法为 `sudo jvman enable {version}`.

## 自行构建

``` cmd
cargo build --release
```

将编译好的 `jvman.exe` 文件拖拽到单独的文件夹，新建 `versions.toml` 文件, `temp` 和 `java` 文件夹， 向 Path 中添加 jvman.exe 的文件路径和`{jvman.exe所在文件夹路径}/OpenJDK/bin` 两个项目，即可正常使用。