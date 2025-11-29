[Defines]
  PLATFORM_NAME           = KernelEFI
  PLATFORM_GUID           = 12345678-1234-1234-1234-1234567890AB
  PLATFORM_VERSION        = 1.0
  DSC_SPEC                = 0x00010005
  OUTPUT_DIRECTORY        = Build/KernelEFI
  SUPPORTED_ARCHITECTURES = AARCH64
  BUILD_TARGETS           = DEBUG|RELEASE

[LibraryClasses]
  UefiLib|MdePkg/Library/UefiLib/UefiLib.inf
  UefiBootServicesTableLib|MdePkg/Library/UefiBootServicesTableLib/UefiBootServicesTableLib.inf
  BaseLib|MdePkg/Library/BaseLib/BaseLib.inf
  BaseMemoryLib|MdePkg/Library/BaseMemoryLib/BaseMemoryLib.inf
  MemoryAllocationLib|MdeModulePkg/Library/BaseMemoryAllocationLibNull/BaseMemoryAllocationLibNull.inf
  PrintLib|MdePkg/Library/BasePrintLib/BasePrintLib.inf
  PcdLib|MdePkg/Library/BasePcdLibNull/BasePcdLibNull.inf
  DebugLib|MdePkg/Library/BaseDebugLibNull/BaseDebugLibNull.inf
  DevicePathLib|MdePkg/Library/UefiDevicePathLib/UefiDevicePathLib.inf
  UefiRuntimeServicesTableLib|MdePkg/Library/UefiRuntimeServicesTableLib/UefiRuntimeServicesTableLib.inf


[Components]
  KernelEFI.inf

