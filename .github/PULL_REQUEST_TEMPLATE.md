## Summary

<!--
請簡述這個 PR 的目的與背景。

請說明：
- 為什麼需要這個修改？
- 解決了什麼問題？
- 希望達成什麼結果？

如果是規則更新，請說明該規則主要針對：
- 廣告
- Tracker / Telemetry
- 惡意網站
- 廣告聯播網
- 其他不需要的網域或服務

請避免只寫「更新規則」、「新增網域」等過於籠統的描述。
-->

## Changes

<!--
請列出這個 PR 實際修改的內容。

請具體說明：
- 修改了哪些檔案？
- 新增、修改或移除了哪些規則？
- 規則屬於哪個平台？
- 是否涉及 Allow List / Block List？
- 是否調整現有規則或規則格式？
-->

*

## Rule Details

<!--
如果本 PR 涉及規則變更，請盡可能提供以下資訊。

### 平台

<!--
請選擇適用的平台：

- AdGuard Home
- Surge
- AdGuard Home + Surge
- 其他
-->

### 規則類型

<!--
例如：

- Block List
- Allow List
- 廣告阻擋
- Tracker 阻擋
- Telemetry 阻擋
- Malware / Phishing
- 其他
-->

### 來源

<!--
請提供規則來源或你判斷需要封鎖 / 允許該網域的依據。

例如：
- 官方文件
- 網站行為
- DNS 查詢
- 網路請求分析
- Browser DevTools
- Surge 日誌
- AdGuard Home Query Log
-->

### 影響範圍

<!--
請說明：
- 哪些服務或網站會受到影響？
- 是否可能造成誤封鎖？
- 是否可能影響登入、影片播放、通知、API 或其他功能？
- 是否需要搭配 Allow List？
-->

## Testing

<!--
請說明你如何驗證此次規則修改。

如果有測試，請提供：
- 測試環境
- 測試方法
- 測試結果

建議確認：

- 規則格式可以被目標平台正常解析
- Block Rule 確實可以阻擋預期網域
- Allow Rule 不會意外放行其他不應允許的網域
- 沒有造成明顯的 False Positive
- 不會影響主要網站或服務的正常使用
- 如果是 Surge 規則，確認規則可以正常載入
- 如果是 AdGuard Home 規則，確認規則可以正常更新
-->

## Breaking Changes

<!--
請確認這個 PR 是否可能改變現有使用者的網路行為。

如果有 Breaking Change，請明確說明：
- 哪些網站、服務或功能可能受到影響？
- 使用者需要做什麼調整？
- 是否可能需要將特定網域加入 Allow List？
- 是否會導致既有連線或服務無法使用？

如果沒有 Breaking Change，請填寫「無」。
-->

## Related Issues

<!--
如果這個 PR 與 Issue、Discussion 或其他 PR 有關，請在此列出。

例如：

- Relates to #123
- Fixes #123
- Closes #123

如果沒有相關項目，請填寫「無」。
-->

## Additional Notes

<!--
請提供 Reviewer 需要知道的其他資訊。

例如：
- 已知限制
- 可能的 False Positive
- 尚未驗證的情況
- 特定網站或服務的相容性問題
- 為什麼使用目前的規則寫法
- 為什麼某個網域應該加入 Block List / Allow List
- 是否需要 Reviewer 特別測試某個網站或服務
-->

<details>
<summary>Checklist</summary>

* [ ] 我已確認這個 PR 的目的與修改內容已清楚說明
* [ ] 我已確認規則格式符合目標平台規範
* [ ] 我已確認規則內容與 PR 目的相符
* [ ] 我已自行 Review 此次變更
* [ ] 我已確認新增的網域或規則確實有阻擋 / 允許的必要性
* [ ] 我已確認規則來源或判斷依據可靠
* [ ] 我已確認沒有明顯的 False Positive
* [ ] 我已確認沒有誤傷主要網站、服務或必要功能
* [ ] 我已確認 Allow List 與 Block List 之間沒有明顯衝突
* [ ] 我已完成適當的本機或實際環境測試
* [ ] 我已確認 AdGuard Home / Surge 可以正常載入修改後的規則（如適用）
* [ ] 我已確認 GitHub Actions / CI 通過（如適用）
* [ ] 我已確認 Release / 自動化流程正常（如適用）
* [ ] 我已確認沒有引入 Breaking Change，或已在上方明確說明
* [ ] 我已確認相關文件已新增或更新（如適用）
* [ ] 我已確認沒有提交不必要的檔案、Debug code 或暫存內容
* [ ] 我已確認變更範圍與 PR 目的相符，沒有混入無關修改
* [ ] 我已確認 Commit Message 符合 Conventional Commits 規範
* [ ] 我已確認沒有洩漏密碼、Token、Cookie、私密金鑰或其他敏感資訊
* [ ] 我已確認 Reviewer 可以根據 PR 說明理解這次變更

</details>

