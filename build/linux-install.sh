#!/bin/bash
# **********************************************************
# * Author      : smile-e3
# * Email       : alchemist_clb@163.com
# * Create time : 2024-04-02
# * Update time : 2024-04-02
# * Filename    : linux_install.sh
# * Description : linux下LibAFL自动化安装
# **********************************************************

source b-log.sh
source common.sh

LOG_LEVEL_ALL
BUILD_PATH=$PWD

#######################################
# 依赖检测函数
# Globals:
#   None
# Arguments:
#   None
# Returns:
#   None
#######################################
dependency_detection(){

    # 依赖工具
    tool_dependencies=("curl")
    result=$(check_commands_existence "${tool_dependencies[@]}")
    if [ "$result" = "True" ]; then
        INFO "依赖工具都已安装"
    else
        for cmd in $result
        do
            sudo apt install -y ${cmd}
            # 检查命令返回值
            if [ $? -ne 0 ]; then
                ERROR "${cmd}安装失败,请检查错误信息"
                exit 0
            fi
        done
    fi

    # 安装RUST的cargo make
    INFO "正在安装RUST环境"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
    # TODO1:仅针对bash终端
    source ~/.bashrc

    # fix1:没有source ~/.bashrc无法查找到命令cargo
    cargo install cargo-make

    # 安装AFL++的LLVM等依赖
    INFO "正在安装LLVM等依赖环境"
    sudo apt-get install -y build-essential python3-dev automake cmake git flex bison libglib2.0-dev libpixman-1-dev python3-setuptools libgtk-3-dev
    sudo apt-get install -y lld-15 llvm-15 llvm-15-dev clang-15
    # 判断能否安装LLVM15
    if [ $? -eq 0 ];then
        INFO "LLVM 15安装成功"
    else
        ERROR "LLVM 15不存在,目前未测试其他版本"
        exit 0
    fi
    # ubuntu 22.04默认GCC 11
    sudo apt-get install -y gcc-$(gcc --version|head -n1|sed 's/\..*//'|sed 's/.* //')-plugin-dev libstdc++-$(gcc --version|head -n1|sed 's/\..*//'|sed 's/.* //')-dev
    sudo apt-get install -y ninja-build # for QEMU mode

    # 安装并构建AFL++
    INFO "正在构建并安装AFL++"
    git clone https://github.com/AFLplusplus/AFLplusplus $HOME/AFLplusplus && cd $HOME/AFLplusplus
    export LLVM_CONFIG="llvm-config-15"
    # 安装AFL++的依赖
    sudo apt install -y python3-pip 
    # TODO2:目前不涉及使用
    # ar rcs libxdc.a build/cfg.o build/disassembler.o build/tnt_cache.o build/decoder.o build/libxdc.o build/mmh3.o build/trace_cache.o
    # /usr/bin/ld: cannot find -l:libcapstone.so.4: No such file or directory
    # collect2: error: ld returned 1 exit status
    # make[1]: *** [Makefile:25: libxdc.so] Error 1
    # make[1]: *** Waiting for unfinished jobs....
    # make[1]: Leaving directory '/home/smile/AFLplusplus/nyx_mode/QEMU-Nyx/libxdc'
    # [ ] libxdc non-LTO build failed again ...
    # make: [GNUmakefile:653: distrib] Error 1 (ignored)
    source ~/.bashrc && make distrib
    if [ $? -eq 0 ];then
        INFO "AFLplusplus安装编译成功"
    else
        ERROR "AFLplusplus安装编译失败"
        exit 0
    fi

    # 将命令安装到本地并返回到脚本目录
    sudo make install && cd $BUILD_PATH
}


#######################################
# 入口函数
# Globals:
#   None
# Arguments:
#   None
# Returns:
#   None
#######################################
main(){

    # 
    sudo apt-get update -y

    # step1:检测依赖
    dependency_detection

}

main