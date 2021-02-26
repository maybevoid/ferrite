use crate::internal::{
  base::*,
  functional::*,
};

pub struct InternalChoice<Row>
where
  Row : RowCon,
{
  pub(crate) field : AppliedSum<Row, ReceiverF>,
}

impl<Row> Protocol for InternalChoice<Row>
where
  Row : Send + 'static,
  Row : RowCon,
{
}

impl<Row, A> RecApp<A> for InternalChoice<Row>
where
  Row : RowCon,
  Row : RecApp<A>,
  Row::Applied : RowCon,
{
  type Applied = InternalChoice<Row::Applied>;
}

impl<Row, A> SharedRecApp<A> for InternalChoice<Row>
where
  Row : RowApp<ReceiverF>,
  Row : SharedRecApp<A>,
  <Row as SharedRecApp<A>>::Applied : RowApp<ReceiverF>,
{
  type Applied = InternalChoice<<Row as SharedRecApp<A>>::Applied>;
}

impl<Row> ForwardChannel for InternalChoice<Row>
where
  Row : RowCon,
  AppliedSum<Row, ReceiverF> : ForwardChannel,
{
  fn forward_to(
    self,
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  )
  {
    self.field.forward_to(sender, receiver)
  }

  fn forward_from(
    sender : OpaqueSender,
    receiver : OpaqueReceiver,
  ) -> Self
  {
    InternalChoice {
      field : <AppliedSum<Row, ReceiverF>>::forward_from(sender, receiver),
    }
  }
}