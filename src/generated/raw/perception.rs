// automatically generated by the FlatBuffers compiler, do not modify


// @generated
extern crate flatbuffers;

pub enum PerceptionOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Perception: What the environment sends to the agent each tick.
/// This must be **agent-agnostic** and include only shared observables.
pub struct Perception<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Perception<'a> {
  type Inner = Perception<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: unsafe { flatbuffers::Table::new(buf, loc) } }
  }
}

impl<'a> Perception<'a> {
  pub const VT_TICK: flatbuffers::VOffsetT = 4;
  pub const VT_ENERGY: flatbuffers::VOffsetT = 6;
  pub const VT_POSITION: flatbuffers::VOffsetT = 8;
  pub const VT_BITFIELD: flatbuffers::VOffsetT = 10;
  pub const VT_NUTRIENT: flatbuffers::VOffsetT = 12;
  pub const VT_CHEMOSENSE: flatbuffers::VOffsetT = 14;
  pub const VT_EXTRA: flatbuffers::VOffsetT = 16;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Perception { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args PerceptionArgs<'args>
  ) -> flatbuffers::WIPOffset<Perception<'bldr>> {
    let mut builder = PerceptionBuilder::new(_fbb);
    if let Some(x) = args.extra { builder.add_extra(x) }
    if let Some(x) = args.chemosense { builder.add_chemosense(x) }
    builder.add_nutrient(args.nutrient);
    if let Some(x) = args.position { builder.add_position(x) }
    builder.add_energy(args.energy);
    builder.add_tick(args.tick);
    builder.add_bitfield(args.bitfield);
    builder.finish()
  }


  #[inline]
  pub fn tick(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(Perception::VT_TICK, Some(0)).unwrap()}
  }
  #[inline]
  pub fn energy(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Perception::VT_ENERGY, Some(0)).unwrap()}
  }
  #[inline]
  pub fn position(&self) -> Option<flatbuffers::Vector<'a, i32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i32>>>(Perception::VT_POSITION, None)}
  }
  #[inline]
  pub fn bitfield(&self) -> u8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u8>(Perception::VT_BITFIELD, Some(0)).unwrap()}
  }
  #[inline]
  pub fn nutrient(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Perception::VT_NUTRIENT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn chemosense(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Perception::VT_CHEMOSENSE, None)}
  }
  #[inline]
  pub fn extra(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Perception::VT_EXTRA, None)}
  }
}

impl flatbuffers::Verifiable for Perception<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    v.visit_table(pos)?
     .visit_field::<u32>("tick", Self::VT_TICK, false)?
     .visit_field::<i32>("energy", Self::VT_ENERGY, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i32>>>("position", Self::VT_POSITION, false)?
     .visit_field::<u8>("bitfield", Self::VT_BITFIELD, false)?
     .visit_field::<i32>("nutrient", Self::VT_NUTRIENT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("chemosense", Self::VT_CHEMOSENSE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("extra", Self::VT_EXTRA, false)?
     .finish();
    Ok(())
  }
}
pub struct PerceptionArgs<'a> {
    pub tick: u32,
    pub energy: i32,
    pub position: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i32>>>,
    pub bitfield: u8,
    pub nutrient: i32,
    pub chemosense: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub extra: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for PerceptionArgs<'a> {
  #[inline]
  fn default() -> Self {
    PerceptionArgs {
      tick: 0,
      energy: 0,
      position: None,
      bitfield: 0,
      nutrient: 0,
      chemosense: None,
      extra: None,
    }
  }
}

pub struct PerceptionBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> PerceptionBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_tick(&mut self, tick: u32) {
    self.fbb_.push_slot::<u32>(Perception::VT_TICK, tick, 0) }
  #[inline]
  pub fn add_energy(&mut self, energy: i32) {
    self.fbb_.push_slot::<i32>(Perception::VT_ENERGY, energy, 0) }
  #[inline]
  pub fn add_position(&mut self, position: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Perception::VT_POSITION, position) }
  #[inline]
  pub fn add_bitfield(&mut self, bitfield: u8) {
    self.fbb_.push_slot::<u8>(Perception::VT_BITFIELD, bitfield, 0) }
  #[inline]
  pub fn add_nutrient(&mut self, nutrient: i32) {
    self.fbb_.push_slot::<i32>(Perception::VT_NUTRIENT, nutrient, 0) }
  #[inline]
  pub fn add_chemosense(&mut self, chemosense: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Perception::VT_CHEMOSENSE, chemosense) }
  #[inline]
  pub fn add_extra(&mut self, extra: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Perception::VT_EXTRA, extra) }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> PerceptionBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    PerceptionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Perception<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Perception<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Perception");
      ds.field("tick", &self.tick());
      ds.field("energy", &self.energy());
      ds.field("position", &self.position());
      ds.field("bitfield", &self.bitfield());
      ds.field("nutrient", &self.nutrient());
      ds.field("chemosense", &self.chemosense());
      ds.field("extra", &self.extra());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Perception`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_perception_unchecked`.
pub fn root_as_perception(buf: &[u8]) -> Result<Perception, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Perception>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Perception` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_perception_unchecked`.
pub fn size_prefixed_root_as_perception(buf: &[u8]) -> Result<Perception, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Perception>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Perception` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_perception_unchecked`.
pub fn root_as_perception_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Perception<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Perception<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Perception` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_perception_unchecked`.
pub fn size_prefixed_root_as_perception_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Perception<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Perception<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Perception and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Perception`.
pub unsafe fn root_as_perception_unchecked(buf: &[u8]) -> Perception {
  unsafe { flatbuffers::root_unchecked::<Perception>(buf) }
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Perception and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Perception`.
pub unsafe fn size_prefixed_root_as_perception_unchecked(buf: &[u8]) -> Perception {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Perception>(buf) }
}
#[inline]
pub fn finish_perception_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Perception<'a>>) {
  fbb.finish(root, None) }

#[inline]
pub fn finish_size_prefixed_perception_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Perception<'a>>) {
  fbb.finish_size_prefixed(root, None) }