# RustIDD
基于 Rust 的 [Indirect display driver](https://learn.microsoft.com/en-us/windows-hardware/drivers/display/indirect-display-driver-model-overview) 实现

## 构建
需要 Windows 10 以上版本
### 依赖安装
由于通过 Windows SDK 和 WDK 版本众多，通过 gui 安装较为麻烦，此处仅提供通过 WinGet 安装的方法，
参考 [Install the WDK using WinGet](https://learn.microsoft.com/en-us/windows-hardware/drivers/install-the-wdk-using-winget)

如果你需要其他安装方法，请参照 [Download the Windows Driver Kit (WDK)](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)

由于 Windows SDK 的版本需要和 WDK 的版本匹配，此处安装的版本固定为 10.0.26100，具体规则参见 [Kit versioning](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk#kit-versioning)
#### Visual Studio 2022
```shell
winget install --source winget --exact --id Microsoft.VisualStudio.2022.Community --override "--passive --config ./wdk.vsconfig"
```
#### Windows SDK
```shell
winget install --source winget --exact --id Microsoft.WindowsSDK.10.0.26100 --log $env:USERPROFILE/Desktop/sdk-install.log
```
#### Windows Driver Kit (WDK)
```shell
winget install --source winget --exact --id Microsoft.WindowsWDK.10.0.26100 --log $env:USERPROFILE/Desktop/wdk-install.log
```
#### Visual Studio WDK 扩展
使用 cmd
```cmd
for /f "usebackq tokens=*" %i in (`"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -nologo -latest -products * -property enginePath`) do (
  "%i\VSIXInstaller.exe" "%ProgramFiles(x86)%\Windows Kits\10\Vsix\VS2022\10.0.26100.0\%PROCESSOR_ARCHITECTURE%\WDK.vsix"
)
```
使用 powershell
```powershell
& $(& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -nologo -latest -products * -property enginePath | Join-Path -ChildPath 'VSIXInstaller.exe') "${env:ProgramFiles(x86)}\Windows Kits\10\Vsix\VS2022\10.0.26100.0\${env:PROCESSOR_ARCHITECTURE}\WDK.vsix"
```

#### LLVM
安装 LLVM 务必会弹出 gui 界面，务必勾选添加到 PATH 的选项
```shell
winget install -i LLVM.LLVM --version 17.0.6 --force
```

#### cargo-make
```shell
cargo install --locked cargo-make --no-default-features --features tls-native
```
### 构建

## 参考
1. [虚拟显示器终极解决方案 **IndirectDisplay**](https://mozz.ie/posts/ultimate-virtual-monitor-solution-indirect-display/)
2. [Windows IDD sample](https://github.com/Microsoft/Windows-driver-samples/tree/main/video/IndirectDisplay)