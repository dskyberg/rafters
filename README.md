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

## Width of the Raftor

Kinda self explanatory, right?
