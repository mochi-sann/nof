use std::path::Path;
use crate::read_package_json::get_scripts;

/// Generate custom completion script for zsh
pub fn generate_zsh_completion() -> String {
    r#"#compdef nof

_nof() {
    local context state line
    local -a options
    
    _arguments -C \
        '1: :_nof_commands' \
        '*:: :->args'
    
    case $state in
        args)
            case $words[1] in
                run|r|R|run-script)
                    _nof_scripts
                    ;;
            esac
            ;;
    esac
}

_nof_commands() {
    local -a commands
    commands=(
        'completion:Generates a script for completion'
        'run:Run node scripts'
        'install:Installs all dependencies'
        'add:Installs a package'
        'remove:remove a package'
        'execute-command:Run a command from a local or remote npm package'
        'help:Print help'
    )
    _describe 'command' commands
}

_nof_scripts() {
    local -a scripts
    # Find package.json file in current directory
    local package_json="./package.json"
    
    # Also check if --target-path was specified earlier in the command line
    local target_path_idx
    for ((i=1; i<${#words[@]}; i++)); do
        if [[ "${words[i]}" == "--target-path" && i+1 -le ${#words[@]} ]]; then
            package_json="${words[i+1]}"
            break
        elif [[ "${words[i]}" =~ --target-path= ]]; then
            package_json="${words[i]#--target-path=}"
            break
        fi
    done
    
    if [[ -f "$package_json" ]]; then
        # Get script completions using nof itself
        scripts=(${(f)"$(nof completion-scripts --target-path "$package_json" 2>/dev/null)"})
        if (( ${#scripts[@]} > 0 )); then
            _describe 'script' scripts
        fi
    fi
}

_nof "$@"
"#.to_string()
}

/// Generate custom completion script for fish
pub fn generate_fish_completion() -> String {
    r#"function __nof_complete_scripts
    set -l package_json ./package.json
    
    # Check if --target-path was specified in command line
    set -l cmd_line (commandline -op)
    for i in (seq (count $cmd_line))
        if test "$cmd_line[$i]" = "--target-path"
            and test (math $i + 1) -le (count $cmd_line)
            set package_json $cmd_line[(math $i + 1)]
            break
        else if string match -q -- '--target-path=*' $cmd_line[$i]
            set package_json (string replace '--target-path=' '' $cmd_line[$i])
            break
        end
    end
    
    if test -f $package_json
        nof completion-scripts --target-path $package_json 2>/dev/null
    end
end

complete -c nof -n "__fish_use_subcommand" -a "completion" -d "Generates a script for completion"
complete -c nof -n "__fish_use_subcommand" -a "run r R run-script" -d "Run node scripts"
complete -c nof -n "__fish_use_subcommand" -a "install i I" -d "Installs all dependencies"
complete -c nof -n "__fish_use_subcommand" -a "add a A" -d "Installs a package"
complete -c nof -n "__fish_use_subcommand" -a "remove rm" -d "remove a package"
complete -c nof -n "__fish_use_subcommand" -a "execute-command e exec E" -d "Run a command from a local or remote npm package"

complete -c nof -n "__fish_seen_subcommand_from run r R run-script" -a "(__nof_complete_scripts)" -d "Package.json script"
"#.to_string()
}

/// Generate custom completion script for bash
pub fn generate_bash_completion() -> String {
    r#"_nof() {
    local cur prev opts package_json="./package.json"
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Check if --target-path was specified earlier in command line
    for ((i=1; i<COMP_CWORD; i++)); do
        if [[ "${COMP_WORDS[i]}" == "--target-path" && i+1 -lt ${#COMP_WORDS[@]} ]]; then
            package_json="${COMP_WORDS[i+1]}"
            break
        elif [[ "${COMP_WORDS[i]}" =~ --target-path= ]]; then
            package_json="${COMP_WORDS[i]#--target-path=}"
            break
        fi
    done

    case ${COMP_CWORD} in
        1)
            opts="completion run install add remove execute-command help"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        *)
            # Handle script completion for run command and its aliases
            local run_cmd_found=false
            for ((i=1; i<COMP_CWORD; i++)); do
                case "${COMP_WORDS[i]}" in
                    run|r|R|run-script)
                        run_cmd_found=true
                        break
                        ;;
                esac
            done
            
            if [[ "$run_cmd_found" == true ]]; then
                # Check if current word is not an option
                if [[ "$cur" != -* ]]; then
                    # Check if previous words contain script argument position
                    local script_position=false
                    local option_count=0
                    for ((i=2; i<COMP_CWORD; i++)); do
                        if [[ "${COMP_WORDS[i]}" == --* ]]; then
                            ((option_count++))
                            if [[ "${COMP_WORDS[i]}" == "--target-path" || "${COMP_WORDS[i]}" == "--package-manneger" ]]; then
                                ((i++)) # Skip the option value
                            fi
                        else
                            script_position=true
                            break
                        fi
                    done
                    
                    if [[ "$script_position" == false ]]; then
                        if [[ -f "$package_json" ]]; then
                            local scripts=$(nof completion-scripts --target-path "$package_json" 2>/dev/null)
                            COMPREPLY=( $(compgen -W "${scripts}" -- ${cur}) )
                        fi
                        return 0
                    fi
                fi
            fi
            ;;
    esac
}

complete -F _nof nof
"#.to_string()
}

/// Get completion candidates for package.json scripts
pub fn get_script_completions(package_json_path: &Path) -> Vec<String> {
    // Handle case where package.json doesn't exist or can't be read
    if !package_json_path.exists() {
        return vec![];
    }
    
    match std::panic::catch_unwind(|| get_scripts(package_json_path)) {
        Ok(scripts) => scripts.into_iter().map(|(name, _)| name).collect(),
        Err(_) => vec![], // Return empty vec if parsing fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_get_script_completions() {
        let file_path = Path::new("./test_files/aaa.json");
        let completions = get_script_completions(file_path);
        
        assert_eq!(completions.len(), 3);
        assert!(completions.contains(&"build".to_string()));
        assert!(completions.contains(&"start".to_string()));
        assert!(completions.contains(&"test".to_string()));
    }
    
    #[test]
    fn test_get_script_completions_nonexistent_file() {
        let file_path = Path::new("./nonexistent.json");
        let completions = get_script_completions(file_path);
        
        assert_eq!(completions.len(), 0);
    }
}