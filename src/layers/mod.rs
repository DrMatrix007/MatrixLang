use std::marker::PhantomData;

pub trait Layer<Input, Output> {
    fn run_layer(&mut self, data: Input) -> Output;
}

pub struct LayerContainer<Input, Output, T: Layer<Input, Output>> {
    layer: T,
    marker: PhantomData<(Input, Output)>,
}

impl<Input, Output, T: Layer<Input, Output>> LayerContainer<Input, Output, T> {
    pub fn new(layer: T) -> Self {
        Self {
            layer,
            marker: PhantomData,
        }
    }

    pub fn run_layer(&mut self, data: Input) -> Output {
        self.layer.run_layer(data)
    }
}

pub trait IntoLayer<Input, Output>: Layer<Input, Output> + Sized {
    fn into_layer(self) -> LayerContainer<Input, Output, Self>;
}

impl<Input, Output, T: Layer<Input, Output>> IntoLayer<Input, Output> for T {
    fn into_layer(self) -> LayerContainer<Input, Output, Self> {
        LayerContainer {
            layer: self,
            marker: PhantomData,
        }
    }
}

pub trait Layers<Input, Output> {
    fn run_layers(&mut self, data: Input) -> Output;
}

impl<Input, Output, Middle, A: Layer<Input, Middle>, B: Layer<Middle, Output>> Layers<Input, Output>
    for (
        LayerContainer<Input, Middle, A>,
        LayerContainer<Middle, Output, B>,
    )
{
    fn run_layers(&mut self, data: Input) -> Output {
        let (a, b) = self;

        let res = a.run_layer(data);

        b.run_layer(res)
    }
}

#[macro_export]
macro_rules! make_layers {
    ($value:expr, $($values:expr),+ $(,)?) => {
        {
            use $crate::layers::IntoLayer;
            (IntoLayer::into_layer($value), make_layers!($($values),*))
        }
    };
    ($value:expr $(,)?) => {
        {
            use $crate::layers::IntoLayer;
            IntoLayer::into_layer($value)
        }
    }
}
