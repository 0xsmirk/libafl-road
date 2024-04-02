#!/bin/bash
# **********************************************************
# * Author      : smile-e3
# * Email       : alchemist_clb@163.com
# * Create time : 2024-04-02
# * Update time : 2024-04-02
# * Filename    : common.sh
# * Description : shell通用库,包含文件检测、命令检测等
# **********************************************************


#######################################
# 检查命令是否存在
# Globals:
#   None
# Arguments:
#   commands
# Returns:
#   bool:True
#   array:missing_commands
#######################################
check_commands_existence() {
    local missing_commands=()
    local commands=("$@")
    
    for cmd in "${commands[@]}"
    do
        if ! command -v "$cmd" &>/dev/null; then
            missing_commands+=("$cmd")
        fi
    done
    
    if [ ${#missing_commands[@]} -eq 0 ]; then
        echo "True"
    else
        echo "${missing_commands[@]}"
    fi
}

#######################################
# 检查文件夹是否存在
# Globals:
#   None
# Arguments:
#   folders
# Returns:
#   bool:True
#   array:missing_folders
#######################################
check_folders_existence() {
    local folders=("$@")
    local missing_folders=()

    for folder in "${folders[@]}"
    do
        if [ ! -d "$folder" ]; then
            missing_folders+=("$folder")
        fi
    done

    if [ ${#missing_folders[@]} -eq 0 ]; then
        echo "True"
    else
        echo "${missing_folders[@]}"
    fi
}