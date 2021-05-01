src = [
  'camera.cc',
  'colour.cc',
  'hittable.cc',
  'hittable_hitrecord.cc',
  'hittable_sphere.cc',
  'lib.cc',
  'main.cc',
  'material_dielectric.cc',
  'material_lambertian.cc',
  'material_metal.cc',
  'perlin.cc',
  'texture_chequer.cc',
  'texture_image.cc',
  'texture_noise.cc',
  'texture_solidcolour.cc',
  'vec3.cc'
]

thread_dep = dependency('threads')

executable('rays', src, dependencies : thread_dep, install : true)
