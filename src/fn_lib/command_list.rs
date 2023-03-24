pub struct NpmCoomands<'a> {
    pub command_name: &'a str,
    pub run: &'a str,
    pub isntall: &'a str,
    pub add: &'a str,
    pub save_dev: &'a str,
    pub save_peer: &'a str,
    pub save_optional: &'a str,
}

pub struct CommandList<'a> {
    pub npm: NpmCoomands<'a>,
    pub yarn: NpmCoomands<'a>,
    pub pnpm: NpmCoomands<'a>,
}

pub const YARN: NpmCoomands = NpmCoomands {
    command_name: "yarn",
    run: "run",
    isntall: "install",
    add: "add",
    save_dev: "-D",
    save_peer: "-P",
    save_optional: "-O",
};

pub const NPM: NpmCoomands = NpmCoomands {
    command_name: "npm",
    run: "run",
    isntall: "install",
    add: "add",
    save_dev: "--save-dev",
    save_peer: "--save-peer",
    save_optional: "--save-optional",
};

pub const PNPM: NpmCoomands = NpmCoomands {
    command_name: "pnpm",
    run: "run",
    isntall: "install",
    add: "add",
    save_dev: "--save-dev",
    save_peer: "--save-peer",
    save_optional: "--save-optional",
};

pub const COMMAND_LIST: CommandList = CommandList {
    npm: NPM,
    yarn: YARN,
    pnpm: PNPM,
};
