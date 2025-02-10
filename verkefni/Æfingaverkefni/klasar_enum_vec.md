# Æfingaverkefni - `Vec` og `Result`

## Breytingar

Breyttu hestinum úr [þessu æfingaverkefni](./klasar_enum.md) þannig að `Stada` hafi eingöngu *Laus*, *Leigdur* og *EkkiTilLeigu*. Breyttu svo *From* yfir í *TryFrom*. Gerðu svo eftirfarandi breytingar á `Hestur`: Láttu *new* fallið skila `Result<Self, String>` og gerðu viðeigandi breytingar á því. Breyttu *breyta_stodu* fallingu þannig að það skili `Result<(), String>`.

## Viðbætur

Gerðu struct-ið *Hestaleiga* sem heldur geymir um `Vec` af hestum ásamt því að sjá um að úthluta viðeigandi id á hestana. *Hestaleiga* þarf að eiga föll til að leigja hesta og skila (afleigja) þeim ásamt falli til að skrá nýjan hest. Hestaleiga þarf svo að útfæra Display.

Gerðu því næst snyrtilegt notendaviðmót, svipað og gert var fyrir [bílasöluna](../../Timar/bilasala/src/main.rs).

## Dæmi um virkni

```bash
>>> skrá Gráni tskóli laus
Gat ekki breytt tskóli í aldur!
>>> skrá Gráni 10 tskóli
Stada: Gat ekki breytt tskóli í stöðu!
>>> skrá Grána 10 laus
>>> skrá Blésa 3 ekki til leigu
>>> skrá Sleipnir 15 laus
>>> prenta
id: 1001, nafn: Grána, aldur: 10, staða: Laus
id: 1002, nafn: Blésa, aldur: 3, staða: Ekki til útleigu
id: 1003, nafn: Sleipnir, aldur: 15, staða: Laus
>>> leigja 1005 
Hestaleiga: breyta_stodu: Fann engan hest með id: 1005
>>> leigja 1001
>>> prenta
id: 1001, nafn: Grána, aldur: 10, staða: Leigður
id: 1002, nafn: Blésa, aldur: 3, staða: Ekki til útleigu
id: 1003, nafn: Sleipnir, aldur: 15, staða: Laus
```

# Lausn

Dæmi um lausn á verkefninu er [hér](./lausn_klasar_enum_vec/src/).

