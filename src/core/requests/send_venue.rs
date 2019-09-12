use crate::core::network;
use crate::core::requests::{
    ChatId, Request, RequestContext, RequestFuture, ResponseResult,
};
use crate::core::types::{Message, ReplyMarkup};

//TODO:: need implementation
///Use this method to send information about a venue. On success, the sent
/// Message is returned.
#[derive(Debug, Clone, Serialize)]
struct SendVenue<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///    Integer or String 	Yes 	Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    ///    Float number 	Yes 	Latitude of the venue
    pub latitude: f64,
    ///Float number 	Yes 	Longitude of the venue
    pub longitude: f64,
    ///    Yes 	Name of the venue
    pub title: String,
    ///String 	Yes 	Address of the venue
    pub address: String,
    ///    String 	Optional 	Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    ///    String 	Optional 	Foursquare type of the venue, if known. (For
    /// example, “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    ///    Boolean 	Optional 	Sends the message silently. Users will receive a
    /// notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    ///    Integer 	Optional 	If the message is a reply, ID of the original
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    ///	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or
    /// ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> Request<'a> for SendVenue<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "sendVenue",
                Some(&self),
            )
            .await
        })
    }
}

impl<'a> SendVenue<'a> {
    pub fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn longitude<T>(mut self, longitude: T) -> Self
    where
        T: Into<f64>,
    {
        self.longitude = longitude.into();
        self
    }

    pub fn latitude<T>(mut self, latitude: T) -> Self
    where
        T: Into<f64>,
    {
        self.latitude = latitude.into();
        self
    }

    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn address<T>(mut self, address: T) -> Self
    where
        T: Into<String>,
    {
        self.address = address.into();
        self
    }

    pub fn foursquare_id<T>(mut self, foursquare_id: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn foursquare_type<T>(mut self, foursquare_type: T) -> Self
        where
            T: Into<bool>,
    {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }
}
