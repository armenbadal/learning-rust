use crate::framebuffer::Framebuffer;

// TODO 2: Գծի ռաստերացում (wireframe)
// Ավելացնել `ScreenLine` տիպ և `Rasterizer`-ի impl՝ Bresenham կամ DDA ալգորիթմով։
// Գրել `Framebuffer::set_pixel`-ով՝ z-ի հետ, որպեսզի խորությունը ճիշտ աշխատի։

// TODO 3: Եռանկյան ռաստերացում
// Ավելացնել `ScreenTriangle` տիպ և լցնել edge function / barycentric եղանակով։
// Յուրաքանչյուր պիքսելի համար ինտերպոլացնել z-ը և ստուգել z-buffer-ով։

// TODO 5: Backface culling
// Եռանկյունը գծելուց առաջ ստուգել winding order-ը (եզրի ֆունկցիայի նշանը)
// և բաց թողնել հետևի կողմը (կամ ավելացնել cull mode)։

// TODO 6: Perspective-correct interpolation
// Գույնը և խորությունը ինտերպոլացնել 1/w-ով (barycentric-ը բաժանել w-ով),
// այլապես perspective-ը կաղավաղվի։

pub trait Rasterizer {
    fn rasterize(&self, fb: &mut Framebuffer);
}