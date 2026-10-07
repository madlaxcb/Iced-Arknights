# 病毒扫描报告（VIRUS-SCAN）

扫描对象：`hud-ui 0.1.0` 发行包中的全部可执行文件。

## 扫描环境

| 项目 | 值 |
|---|---|
| 扫描引擎 | ClamAV `clamscan` 1.4.3 |
| 病毒特征库 | main.cvd + daily.cvd（共 3,628,131 条签名），更新于 2026-10-06 07:20 UTC+8 |
| 扫描时间 | 2026-10-06 22:01:04 – 22:01:33（本机时区） |
| 扫描命令 | `clamscan linux-x86_64/gallery linux-x86_64/sample-app windows-x86_64/gallery.exe windows-x86_64/sample-app.exe` |

## 逐文件结果

| 文件 | SHA-256 | 结果 |
|---|---|---|
| `linux-x86_64/gallery` | `e44309b5db51f514b7179c31bf5a385288999b05f207873a6b4609f0b6586b02` | OK（未发现威胁） |
| `linux-x86_64/sample-app` | `92247ec695cf07a2e8e827b91035cc9eddf7bbe2fc41e5e6db5903c65aea6fad` | OK（未发现威胁） |
| `windows-x86_64/gallery.exe` | `f16f5f37da9954160ee27db3e14d70d6d04f7491bfceb34a4a40dad4abad21f1` | OK（未发现威胁） |
| `windows-x86_64/sample-app.exe` | `4f3bfce78f9b89ac5460cac06faa35d2ee6e56a558409de08af43b70a441fb87` | OK（未发现威胁） |

汇总：扫描文件 4，**感染文件 0**。完整汇总见同目录 `CHECKSUMS.sha256`
（可用 `sha256sum -c CHECKSUMS.sha256` 校验下载完整性）。

## 说明

- 以上为单一引擎（ClamAV）在本机构建环境中的扫描结果，仅作参考，
  不构成任何安全担保。
- 本程序为未签名的 Rust 交叉编译产物，部分杀毒软件可能对无签名可执行文件
  产生启发式误报；如遇告警，建议先核对 SHA-256 是否与本报告一致，
  再将文件提交至 [VirusTotal](https://www.virustotal.com/) 多引擎复核。
