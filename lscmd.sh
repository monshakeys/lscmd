# shellcheck shell=zsh
# ==============================================================================
# lscmd: 列出 Shell 命令 (別名與函式)
#
# 使用方式:
#   lscmd
#
# 特性:
#   - 可讀性優化：設定置頂、增加註釋、語意化變數。
#   - 顏色區分：檔名、Alias、Function 使用不同顏色，為黑底終端機優化。
#   - 排版優化：預設為五欄位平均分配佈局。
#   - 排序：檔名與檔案內的指令均按字母順序排序。
# ==============================================================================
lscmd() {
  # 如果 Homebrew 已安裝 lscmd 的話便直接呼叫
  if brew --prefix lscmd >/dev/null 2>&1; then
    command lscmd "$@"
    return
  fi

  # --- 腳本設定區 ---

  # 1. 根目錄路徑設定 (Root path is set at the top for clarity)
  local ROOT_PATH="$HOME/.alias/"

  # 2. 顏色代碼 (Colors optimized for dark terminals)
  local BOLD="\033[1m"
  #  檔名使用橘色，alias 使用綠色，function 使用藍色
  local C_ORANGE="\033[38;5;214m" # Orange for filename
  local C_GREEN="\033[38;5;48m"   # Green for alias
  local C_BLUE="\033[38;5;39m"  # Blue for function
  local C_RESET="\033[0m"     # Reset color

  # 3. 排版欄位數 (Layout is set to five columns)
  local NUM_COLS=5

  # --- 腳本核心邏輯 ---

  # 暫時關閉 xtrace (如果已開啟)，並在函式結束時恢復
  local xtrace_was_on=0
  if [[ -o xtrace ]]; then
    xtrace_was_on=1
    set +x
  fi

  # 使用 emulate 確保在 zsh 環境下行為一致
  emulate -L zsh
  # 將陣列索引設為 0-based，以匹配 C-style for 迴圈
  setopt KSH_ARRAYS

  # 檢查根目錄是否存在
  if [[ ! -d "$ROOT_PATH" ]]; then
    echo "lscmd 錯誤: 目錄 '$ROOT_PATH' 不存在。" >&2
    return 1
  fi

  # 找出所有 .sh 檔案並依字母排序
  local files
  files=(${(f)"$(find "$ROOT_PATH" -name "*.sh" -type f | sort)"})

  if [[ ${#files[@]} -eq 0 ]]; then
    echo "lscmd 提示: 在 '$ROOT_PATH' 中找不到任何 .sh 檔案。"
    return 0
  fi

  # 遍歷所有找到的檔案
  for file in "${files[@]}"; do
    local commands_with_type=()

    # 使用 Process Substitution 讀取 awk 的輸出，避免 subshell 問題
    # while 迴圈確保能處理結尾缺少換行符的檔案
    while IFS= read -r line || [[ -n "$line" ]]; do
      [[ -n "$line" ]] && commands_with_type+=("$line")
    done < <(
      # awk 用於解析檔案，提取 alias 和 function 名稱
      # 並在前面加上 "alias:" 或 "func:" 前綴，以便後續上色
      awk '
        /^[[:space:]]*alias / {
          s = $0;
          sub(/=.*/, "", s);
          sub(/^[[:space:]]*alias /, "", s);
          print "alias:" s
        }
        /^[[:space:]]*function / {
          name = $2;
          sub(/\(\).*/, "", name);
          print "func:" name
        }
        /^[a-zA-Z0-9_-]+[[:space:]]*\(\)/ && !/^[[:space:]]*alias / {
          s = $0;
          sub(/[[:space:]]*\(\).*/, "", s);
          print "func:" s
        }
      ' "$file"
    )

    if [[ ${#commands_with_type[@]} -eq 0 ]]; then
      continue
    fi

    # 輸出帶有顏色的檔名標頭
    local filename_no_ext=$(basename "$file" .sh)
    echo -e "\n${C_ORANGE}${BOLD}## ${filename_no_ext:u}${C_RESET}"

    # 根據指令名稱進行排序
    local sorted_commands=(${(f)"$(printf "%s\n" "${commands_with_type[@]}" | sort -t: -k2)"})

    # --- 計算排版佈局 ---
    local total=${#sorted_commands[@]}
    local term_width=$(tput cols 2>/dev/null || echo 90)
    local col_width=$(( (term_width - (NUM_COLS * 2)) / NUM_COLS ))
    local rows=$(( (total + NUM_COLS - 1) / NUM_COLS ))

    # --- 逐行打印 ---
    for ((r = 0; r < rows; r++)); do
      local line_to_print=""
      for ((c = 0; c < NUM_COLS; c++)); do
        local index=$((r + c * rows))
        if [[ $index -lt $total ]]; then
          local prefixed_cmd="${sorted_commands[$index]}"
          local cmd_type="${prefixed_cmd%%:*}"
          local cmd_name="${prefixed_cmd#*:}"

          local color="$C_GREEN"
          if [[ "$cmd_type" == "func" ]]; then
            color="$C_BLUE"
          fi
          local colored_cmd="${color}${cmd_name}${C_RESET}"

          local padding=$((col_width - ${#cmd_name}))
          [[ $padding -le 0 ]] && padding=2

          line_to_print+="${colored_cmd}$(printf '%*s' $padding '')"
        fi
      done
      echo -e "$line_to_print"
    done
  done

  # 恢復之前的 xtrace 設定
  (( xtrace_was_on )) && set -x
}
