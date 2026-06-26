export DIDI_SESSION_ID=$(uuidgen)

_didi_preexec() {
    if [[ "$BASH_COMMAND" != "_didi_precmd" ]]; then
        DIDI_LAST_CMD="$BASH_COMMAND"
    fi
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

trap '_didi_preexec' DEBUG
PROMPT_COMMAND="_didi_precmd${PROMPT_COMMAND:+; $PROMPT_COMMAND}"