use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};

fn main()
{
    // Read the environment variables we set in 'build.rs'
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    // Choose whether to start the UEFI or BIOS image
    let use_uefi = true;

    let mut run_command = std::process::Command::new("qemu-system-x86_64");
    if use_uefi
    {
        // Fetch the latest prebuilt OVMF image
        let prebuilt = Prebuilt::fetch(
            Source::LATEST,
            "target/ovmf"
        ).expect("failed to update prebuilt");
        
        // Get the path to the prebuilt image
        let path_to_prebuilt = prebuilt.get_file(Arch::X64, FileType::Code);
        
        run_command.arg("-bios").arg(path_to_prebuilt);
        run_command.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    }
    else
    {
        run_command.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }

    // Run the 'run_command' in a child process, and wait for the child process to exit
    // before terminating.
    let mut child_process = run_command.spawn().unwrap();
    child_process.wait().unwrap();
}