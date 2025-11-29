#include <Uefi.h>
#include <Library/UefiLib.h>
#include <Library/UefiBootServicesTableLib.h>

extern void rust_main(void);

EFI_STATUS
EFIAPI
UefiMain(IN EFI_HANDLE ImageHandle, IN EFI_SYSTEM_TABLE *SystemTable)
{
    Print(L"Was geht ab in Rumänien\n");

    // Call the Rust kernel. Rust never returns.
    rust_main();

    // Should never get here
    return EFI_SUCCESS;
}
