# Rafters
This program calculates the length of rafters for a roof, along with where to put the
bird's mouth.

Calculating rafters requires calculating 3 triangles.  The first is themain triangle that runs between
the outside edge of the span, to the ridge beam. This determines where the heal of the bird's mouth is placed.
The height of the birds mouth is determined by the width of the rafter and the wall (which includes the top plate and sheathing).
The final triangle is determined by the desired overhang.

The total rafter length is calculated by the length of the main triangle (the hypotenuse) and the length of the overhang.

The total ridge beam height is calculated as the height of the main triangle and the angled width of the rafter,
minus the bird's mouth heal (rise).

## Run

```sh
➜ rafters --help
Usage: rafters --pitch <4, 7, etc> --span <372.25 etc> --wall-width <6.125, etc> --beam-thickness <1.5, etc> --beam-width <1.5, etc> --overhang <18.0, 24.0, etc> --rafter-width <9.25, etc>

Options:
  -p, --pitch <4, 7, etc>           The rise in the rise over run calculation. Such as the 4 in 4:12
  -s, --span <372.25 etc>           Distance in inches between the outside edges of the opposing walls that will hold the rafters. Remember to include the thickness of sheathing
  -w, --wall-width <6.125, etc>     Width of the wall (top plate width + sheathing thickness) in inches, such as 5.5 for a 2x6 plate. This is needed to properly calculate the bird's mouth
  -t, --beam-thickness <1.5, etc>   Thickness of the ridge board or beam in inches. Such as 1.5 for a typical 2x8 ridge board
  -b, --beam-width <1.5, etc>       Width of the ridge board or beam in inches. Such as 11.25 for a typical 6x12 ridge board
  -o, --overhang <18.0, 24.0, etc>  Distance from the tip of the rafter to the outside edge of the wall in inches
  -r, --rafter-width <9.25, etc>    Width of the rafter in inches, such as 9.25 for a 2x10 rafter
  -h, --help                        Print help
```

## Measurements

This app uses inches with decimals.  So, just divide the the fraction to get the decimal.
Thus, `10 3/4` inches becomes `10.75` inches. and `1 5/8` inches becomes `1.625` inches.

## Pitch

It's assumed that the builder will determine the pitch of the roof based on climate and asthetics.
Typical pitches are between 4 and 7.  But it's totally arbitrary to calculating rafters.

## Span

The span is the distance between the outside edge of the walls, including the thickness of the top plate and sheathing.
Thus, for a house with an inside dimension of `30"`, a `5.5'` top plate, and sheathing thickness of `5/8"`,
the span is `30 + (5.5 * 2) + (5/8 * 2) = 36.625"`.

## Width of the Top Plate

Typical top plates are `5.5` inches wide.  However, it's important to remember to add the thickness of the
planned sheathing to the width of the actual top plate.  Thus, if you are using typical `5/8'` sheathing,
the total width of the top plate is `5.5 + 0.625 = 6.125'`.

## Ridge Thickness

If you are using a ridge board, it's likely `1.5"` wide (`2x8` or `2x10`).  If you are actually using a
ridge beam, your mileage may vary.

## Overhang

The overhang is the distance from the edge of the wall to the edge of the rafter. This is where the Sophet is hung.  It's typically between `18` and `24` inches.

## Width of the Rafter

Kinda self explanatory, right?
