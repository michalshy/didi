export DIDI_SESSION_ID=$(uuidgen)

_didi_preexec() {
    DIDI_LAST_CMD="$1"
}

_didi_precmd() {
    local exit_code=$?
    if [[ -n "$DIDI_LAST_CMD" ]]; then
        didi log \
            --cmd="$DIDI_LAST_CMD" \
            --cwd="$PWD" \
            --exit="$exit_code" \
            --session="$DIDI_SESSION_ID" \
            > /dev/null 2>&1
        DIDI_LAST_CMD=""
    fi
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _didi_preexec
add-zsh-hook precmd _didi_precmd