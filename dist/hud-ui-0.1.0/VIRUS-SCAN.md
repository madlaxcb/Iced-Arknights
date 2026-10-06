# 病毒扫描报告（VIRUS-SCAN）

扫描对象：`hud-ui 0.1.0` 发行包中的全部可执行文件。

## 扫描环境

| 项目 | 值 |
|---|---|
| 扫描引擎 | ClamAV `clamscan` 1.4.3 |
| 病毒特征库 | main.cvd + daily.cvd（共 3,628,131 条签名），更新于 2026-10-06 07:20 UTC+8 |
| 扫描时间 | 2026-10-06 07:21:20 – 07:21:48（UTC+8） |
| 扫描命令 | `clamscan linux-x86_64/gallery linux-x86_64/sample-app windows-x86_64/gallery.exe windows-x86_64/sample-app.exe` |

## 逐文件结果

| 文件 | SHA-256 | 结果 |
|---|---|---|
| `linux-x86_64/gallery` | `fd2b692b5e094ac1840b6a23037dd7d98db36edefdb7d0b53b1b5d830651a7b7` | OK（未发现威胁） |
| `linux-x86_64/sample-app` | `3347563843993a7da1444d31aeb3d006fdcf022cfde0d26e22aa0da0b53dd65e` | OK（未发现威胁） |
| `windows-x86_64/gallery.exe` | `8594321dc604169f6d53a2a172fdf15f330d401ac315e3e4aa33f9bdfac5db4f` | OK（未发现威胁） |
| `windows-x86_64/sample-app.exe` | `1d07fe3e41fafe098d77e7ec216cab95a354d8ccbadbd7b28e829ccf5b947ad8` | OK（未发现威胁） |

汇总：扫描文件 4，**感染文件 0**。完整汇总见同目录 `CHECKSUMS.sha256`
（可用 `sha256sum -c CHECKSUMS.sha256` 校验下载完整性）。

## 说明

- 以上为单一引擎（ClamAV）在本机构建环境中的扫描结果，仅作参考，
  不构成任何安全担保。
- 本程序为未签名的 Rust 交叉编译产物，部分杀毒软件可能对无签名可执行文件
  产生启发式误报；如遇告警，建议先核对 SHA-256 是否与本报告一致，
  再将文件提交至 [VirusTotal](https://www.virustotal.com/) 多引擎复核。
