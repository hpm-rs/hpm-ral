FILED = [
    "ADC1",
    "ADC2",
    "ADC3",
    "ACMP",
    "I2S0",
    "I2S1",
    "I2S2",
    "I2S3",
    "PDM",
    "DAO",
    "SYNT",
    "MOTO",
    "MOT1",
    "MOT2",
    "MOT3",
    "LCDC",
    "CAMO",
    "CAM1",
    "JPEG",
    "PDMA",
    "ENETO",
    "ENET1",
    "NTMRO",
    "NTMR1",
    "SDXC0",
    "SDXC1",
    "USBO",
    "USB1",
]


def print_field(name: str, offset: int):
    print(
        f"""{name}:
  bitOffset: {offset}
  bitWidth: 1"""
    )


def main():
    for index, name in enumerate(FILED):
        print_field(name, index)


if __name__ == "__main__":
    main()
