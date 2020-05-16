use crate::base::*;
use async_std::sync::Receiver;

pub trait ProtocolSum2
  : Send + 'static
{
  type ValueSum : Send;

  type SelectCurrent : Nat + Send + 'static;
  type SelectorSum : Send + 'static;

  fn select_current () ->
    Self :: SelectCurrent;

  fn value_sum_to_selector_sum
    ( value_sum: &Self::ValueSum
    ) ->
      Self::SelectorSum;
}

pub trait SelectSum < N > : ProtocolSum2 {
  type SelectedProtocol : Protocol;

  fn inject_selected
    ( receiver :
        Receiver <
          Self :: SelectedProtocol
        >
    ) ->
      Self :: ValueSum;
}

pub struct InternalChoice < Sum >
where
  Sum : ProtocolSum2
{ pub (crate) value_sum:
    Sum :: ValueSum
}

pub struct ExternalChoice < Sum >
where
  Sum : ProtocolSum2
{ pub (crate) cont_sum:
    Box <
      dyn FnOnce
        ( Sum :: SelectorSum
        ) ->
          Sum :: ValueSum
      + Send
    >
}

impl
  < Sum >
  Protocol for
  InternalChoice < Sum >
where
  Sum : ProtocolSum2
{ }

impl
  < Sum >
  Protocol for
  ExternalChoice < Sum >
where
  Sum : ProtocolSum2
{ }

impl < P > ProtocolSum2 for P
where
  P : Protocol
{
  type ValueSum =
    Receiver < P >;

  type SelectCurrent = Z;
  type SelectorSum = Z;

  fn select_current () ->
    Self :: SelectCurrent
  {
    Z {}
  }

  fn value_sum_to_selector_sum
    ( _: &Self::ValueSum
    ) ->
      Self::SelectorSum
  {
    Self :: select_current()
  }
}

impl < P, R >
  ProtocolSum2
  for Sum < P, R >
where
  P : Protocol,
  R : ProtocolSum2,
{
  type ValueSum =
    Sum <
      Receiver < P >,
      R :: ValueSum
    >;

  type SelectCurrent =
    S <
      R :: SelectCurrent
    >;

  type SelectorSum =
    Sum <
      Self::SelectCurrent,
      R :: SelectorSum
    >;

  fn select_current () ->
    Self :: SelectCurrent
  {
    Self :: SelectCurrent :: nat ()
  }

  fn value_sum_to_selector_sum
    ( val_sum: &Self::ValueSum
    ) ->
      Self::SelectorSum
    {
      match val_sum {
        Sum::Inl (_) => {
          Sum::Inl (
            Self :: select_current()
          )
        },
        Sum::Inr (val_sum2) => {
          Sum::Inr (
            R :: value_sum_to_selector_sum
              ( val_sum2) )
        }
      }
    }
}

impl
  < P >
  SelectSum < Z >
  for P
where
  P : Protocol
{
  type SelectedProtocol = P;

  fn inject_selected
    ( receiver :
        Receiver < P >
    ) ->
      Self :: ValueSum
  {
    receiver
  }
}

impl
  < P, R >
  SelectSum <
    Z
  >
  for Sum < P, R >
where
  P : Protocol,
  R : ProtocolSum2,
{
  type SelectedProtocol = P;

  fn inject_selected
    ( receiver :
        Receiver < P >
    ) ->
      Self :: ValueSum
  {
    Sum::Inl ( receiver )
  }
}

impl
  < P, R, N >
  SelectSum <
    S < N >
  >
  for Sum < P, R >
where
  N : Nat,
  P : Protocol,
  R : ProtocolSum2,
  R : SelectSum < N >
{
  type SelectedProtocol =
    < R as SelectSum < N >
    > :: SelectedProtocol ;

  fn inject_selected
    ( receiver :
        Receiver <
          Self :: SelectedProtocol
        >
    ) ->
      Self :: ValueSum
  {
    Sum :: Inr (
      < R as SelectSum < N >
      > :: inject_selected ( receiver )
    )
  }
}