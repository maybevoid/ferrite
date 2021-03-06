#[macro_export]

macro_rules! natural_transformation {
  ( { $( $field:ident : $field_type:ty ),* $(,)? } ;
    $name:ident : forall x . $f1:ty [@x] -> $f2:ty [@x] ;
    ($arg:ident) => $body:expr
  ) => {
    {
      struct $name <'a> {
        $( $field : &'a $field_type ),*
        _phantom : &'a ()
      }

      impl <'a> $crate::internal::functional::NaturalTransformation
        < $f1, $f2 >
      for $name<'a>
      {
        fn lift < A >
          ( &self,
            $arg: App < $f1, A >
          )
          -> App < $f2, A >
        where
          A: Send + 'static
        {
          $body
        }
      }
      &$name{
        $( $field : &$field ),*
        _phantom: &()
      }
    }
  }
}
