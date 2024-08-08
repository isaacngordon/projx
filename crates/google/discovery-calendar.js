const discovery_calendar ={
    "discoveryVersion": "v1",
    "kind": "discovery#restDescription",
    // auth contains the scopes and permissions that this api service supports
    "auth": {
      "oauth2": {
        "scopes": {
          "https://www.googleapis.com/auth/calendar": {
            "description": "See, edit, share, and permanently delete all the calendars you can access using Google Calendar"
          },
          "https://www.googleapis.com/auth/calendar.events": {
            "description": "View and edit events on all your calendars"
          },
          "https://www.googleapis.com/auth/calendar.events.readonly": {
            "description": "View events on all your calendars"
          },
          "https://www.googleapis.com/auth/calendar.readonly": {
            "description": "See and download any calendar you can access using your Google Calendar"
          },
          "https://www.googleapis.com/auth/calendar.settings.readonly": {
            "description": "View your Calendar settings"
          }
        }
      }
    },
    // the base url for all the api requests for this service
    "baseUrl": "https://www.googleapis.com/calendar/v3/",
    // revision is the revision of the api, this is used to track changes to the api
    "revision": "20240726",
    "icons": {
      "x16": "http://fonts.gstatic.com/s/i/productlogos/calendar_2020q4/v8/web-16dp/logo_calendar_2020q4_color_1x_web_16dp.png",
      "x32": "http://fonts.gstatic.com/s/i/productlogos/calendar_2020q4/v8/web-32dp/logo_calendar_2020q4_color_1x_web_32dp.png"
    },
    // name of the api/service
    "name": "calendar",
    "schemas": {
      "Acl": {
        "id": "Acl",
        "type": "object",
        "properties": {
          "etag": {
            "type": "string",
            "description": "ETag of the collection."
          },
          "items": {
            "type": "array",
            "description": "List of rules on the access control list.",
            "items": {
              "$ref": "AclRule"
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the collection (\"calendar#acl\").",
            "default": "calendar#acl"
          },
          "nextPageToken": {
            "type": "string",
            "description": "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."
          },
          "nextSyncToken": {
            "type": "string",
            "description": "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."
          }
        }
      },
      "AclRule": {
        "id": "AclRule",
        "type": "object",
        "properties": {
          "etag": {
            "type": "string",
            "description": "ETag of the resource."
          },
          "id": {
            "type": "string",
            "description": "Identifier of the Access Control List (ACL) rule. See Sharing calendars."
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#aclRule\").",
            "default": "calendar#aclRule"
          },
          "role": {
            "type": "string",
            "description": "The role assigned to the scope. Possible values are:  \n- \"none\" - Provides no access. \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs.",
            "annotations": {
              "required": [
                "calendar.acl.insert"
              ]
            }
          },
          "scope": {
            "type": "object",
            "description": "The extent to which calendar access is granted by this ACL rule.",
            "properties": {
              "type": {
                "type": "string",
                "description": "The type of the scope. Possible values are:  \n- \"default\" - The public scope. This is the default value. \n- \"user\" - Limits the scope to a single user. \n- \"group\" - Limits the scope to a group. \n- \"domain\" - Limits the scope to a domain.  Note: The permissions granted to the \"default\", or public, scope apply to any user, authenticated or not.",
                "annotations": {
                  "required": [
                    "calendar.acl.insert",
                    "calendar.acl.update"
                  ]
                }
              },
              "value": {
                "type": "string",
                "description": "The email address of a user or group, or the name of a domain, depending on the scope type. Omitted for type \"default\"."
              }
            },
            "annotations": {
              "required": [
                "calendar.acl.insert",
                "calendar.acl.update"
              ]
            }
          }
        }
      },
      "Calendar": {
        "id": "Calendar",
        "type": "object",
        "properties": {
          "conferenceProperties": {
            "$ref": "ConferenceProperties",
            "description": "Conferencing properties for this calendar, for example what types of conferences are allowed."
          },
          "description": {
            "type": "string",
            "description": "Description of the calendar. Optional."
          },
          "etag": {
            "type": "string",
            "description": "ETag of the resource."
          },
          "id": {
            "type": "string",
            "description": "Identifier of the calendar. To retrieve IDs call the calendarList.list() method."
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#calendar\").",
            "default": "calendar#calendar"
          },
          "location": {
            "type": "string",
            "description": "Geographic location of the calendar as free-form text. Optional."
          },
          "summary": {
            "type": "string",
            "description": "Title of the calendar.",
            "annotations": {
              "required": [
                "calendar.calendars.insert"
              ]
            }
          },
          "timeZone": {
            "type": "string",
            "description": "The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) Optional."
          }
        }
      },
      "CalendarList": {
        "id": "CalendarList",
        "type": "object",
        "properties": {
          "etag": {
            "type": "string",
            "description": "ETag of the collection."
          },
          "items": {
            "type": "array",
            "description": "Calendars that are present on the user's calendar list.",
            "items": {
              "$ref": "CalendarListEntry"
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the collection (\"calendar#calendarList\").",
            "default": "calendar#calendarList"
          },
          "nextPageToken": {
            "type": "string",
            "description": "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."
          },
          "nextSyncToken": {
            "type": "string",
            "description": "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."
          }
        }
      },
      "CalendarListEntry": {
        "id": "CalendarListEntry",
        "type": "object",
        "properties": {
          "accessRole": {
            "type": "string",
            "description": "The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."
          },
          "backgroundColor": {
            "type": "string",
            "description": "The main color of the calendar in the hexadecimal format \"#0088aa\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."
          },
          "colorId": {
            "type": "string",
            "description": "The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional."
          },
          "conferenceProperties": {
            "$ref": "ConferenceProperties",
            "description": "Conferencing properties for this calendar, for example what types of conferences are allowed."
          },
          "defaultReminders": {
            "type": "array",
            "description": "The default reminders that the authenticated user has for this calendar.",
            "items": {
              "$ref": "EventReminder"
            }
          },
          "deleted": {
            "type": "boolean",
            "description": "Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.",
            "default": "false"
          },
          "description": {
            "type": "string",
            "description": "Description of the calendar. Optional. Read-only."
          },
          "etag": {
            "type": "string",
            "description": "ETag of the resource."
          },
          "foregroundColor": {
            "type": "string",
            "description": "The foreground color of the calendar in the hexadecimal format \"#ffffff\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."
          },
          "hidden": {
            "type": "boolean",
            "description": "Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.",
            "default": "false"
          },
          "id": {
            "type": "string",
            "description": "Identifier of the calendar.",
            "annotations": {
              "required": [
                "calendar.calendarList.insert"
              ]
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#calendarListEntry\").",
            "default": "calendar#calendarListEntry"
          },
          "location": {
            "type": "string",
            "description": "Geographic location of the calendar as free-form text. Optional. Read-only."
          },
          "notificationSettings": {
            "type": "object",
            "description": "The notifications that the authenticated user is receiving for this calendar.",
            "properties": {
              "notifications": {
                "type": "array",
                "description": "The list of notifications set for this calendar.",
                "items": {
                  "$ref": "CalendarNotification"
                }
              }
            }
          },
          "primary": {
            "type": "boolean",
            "description": "Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.",
            "default": "false"
          },
          "selected": {
            "type": "boolean",
            "description": "Whether the calendar content shows up in the calendar UI. Optional. The default is False.",
            "default": "false"
          },
          "summary": {
            "type": "string",
            "description": "Title of the calendar. Read-only."
          },
          "summaryOverride": {
            "type": "string",
            "description": "The summary that the authenticated user has set for this calendar. Optional."
          },
          "timeZone": {
            "type": "string",
            "description": "The time zone of the calendar. Optional. Read-only."
          }
        }
      },
      "CalendarNotification": {
        "id": "CalendarNotification",
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "description": "The method used to deliver the notification. The possible value is:  \n- \"email\" - Notifications are sent via email.  \nRequired when adding a notification."
          },
          "type": {
            "type": "string",
            "description": "The type of notification. Possible values are:  \n- \"eventCreation\" - Notification sent when a new event is put on the calendar. \n- \"eventChange\" - Notification sent when an event is changed. \n- \"eventCancellation\" - Notification sent when an event is cancelled. \n- \"eventResponse\" - Notification sent when an attendee responds to the event invitation. \n- \"agenda\" - An agenda with the events of the day (sent out in the morning).  \nRequired when adding a notification."
          }
        }
      },
      "Channel": {
        "id": "Channel",
        "type": "object",
        "properties": {
          "address": {
            "type": "string",
            "description": "The address where notifications are delivered for this channel."
          },
          "expiration": {
            "type": "string",
            "description": "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.",
            "format": "int64"
          },
          "id": {
            "type": "string",
            "description": "A UUID or similar unique string that identifies this channel."
          },
          "kind": {
            "type": "string",
            "description": "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\".",
            "default": "api#channel"
          },
          "params": {
            "type": "object",
            "description": "Additional parameters controlling delivery channel behavior. Optional.",
            "additionalProperties": {
              "type": "string",
              "description": "Declares a new parameter by name."
            }
          },
          "payload": {
            "type": "boolean",
            "description": "A Boolean value to indicate whether payload is wanted. Optional."
          },
          "resourceId": {
            "type": "string",
            "description": "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."
          },
          "resourceUri": {
            "type": "string",
            "description": "A version-specific identifier for the watched resource."
          },
          "token": {
            "type": "string",
            "description": "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."
          },
          "type": {
            "type": "string",
            "description": "The type of delivery mechanism used for this channel. Valid values are \"web_hook\" (or \"webhook\"). Both values refer to a channel where Http requests are used to deliver messages."
          }
        }
      },
      "ColorDefinition": {
        "id": "ColorDefinition",
        "type": "object",
        "properties": {
          "background": {
            "type": "string",
            "description": "The background color associated with this color definition."
          },
          "foreground": {
            "type": "string",
            "description": "The foreground color that can be used to write on top of a background with 'background' color."
          }
        }
      },
      "Colors": {
        "id": "Colors",
        "type": "object",
        "properties": {
          "calendar": {
            "type": "object",
            "description": "A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only.",
            "additionalProperties": {
              "$ref": "ColorDefinition",
              "description": "A calendar color definition."
            }
          },
          "event": {
            "type": "object",
            "description": "A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only.",
            "additionalProperties": {
              "$ref": "ColorDefinition",
              "description": "An event color definition."
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#colors\").",
            "default": "calendar#colors"
          },
          "updated": {
            "type": "string",
            "description": "Last modification time of the color palette (as a RFC3339 timestamp). Read-only.",
            "format": "date-time"
          }
        }
      },
      "ConferenceData": {
        "id": "ConferenceData",
        "type": "object",
        "properties": {
          "conferenceId": {
            "type": "string",
            "description": "The ID of the conference.\nCan be used by developers to keep track of conferences, should not be displayed to users.\nThe ID value is formed differently for each conference solution type:  \n- eventHangout: ID is not set. (This conference type is deprecated.)\n- eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)\n- hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.\n- addOn: ID is defined by the third-party provider.  Optional."
          },
          "conferenceSolution": {
            "$ref": "ConferenceSolution",
            "description": "The conference solution, such as Google Meet.\nUnset for a conference with a failed create request.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."
          },
          "createRequest": {
            "$ref": "CreateConferenceRequest",
            "description": "A request to generate a new conference and attach it to the event. The data is generated asynchronously. To see whether the data is present check the status field.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."
          },
          "entryPoints": {
            "type": "array",
            "description": "Information about individual conference entry points, such as URLs or phone numbers.\nAll of them must belong to the same conference.\nEither conferenceSolution and at least one entryPoint, or createRequest is required.",
            "items": {
              "$ref": "EntryPoint"
            }
          },
          "notes": {
            "type": "string",
            "description": "Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional."
          },
          "parameters": {
            "$ref": "ConferenceParameters",
            "description": "Additional properties related to a conference. An example would be a solution-specific setting for enabling video streaming."
          },
          "signature": {
            "type": "string",
            "description": "The signature of the conference data.\nGenerated on server side.\nUnset for a conference with a failed create request.\nOptional for a conference with a pending create request."
          }
        }
      },
      "ConferenceParameters": {
        "id": "ConferenceParameters",
        "type": "object",
        "properties": {
          "addOnParameters": {
            "$ref": "ConferenceParametersAddOnParameters",
            "description": "Additional add-on specific data."
          }
        }
      },
      "ConferenceParametersAddOnParameters": {
        "id": "ConferenceParametersAddOnParameters",
        "type": "object",
        "properties": {
          "parameters": {
            "type": "object",
            "additionalProperties": {
              "type": "string"
            }
          }
        }
      },
      "ConferenceProperties": {
        "id": "ConferenceProperties",
        "type": "object",
        "properties": {
          "allowedConferenceSolutionTypes": {
            "type": "array",
            "description": "The types of conference solutions that are supported for this calendar.\nThe possible values are:  \n- \"eventHangout\" \n- \"eventNamedHangout\" \n- \"hangoutsMeet\"  Optional.",
            "items": {
              "type": "string"
            }
          }
        }
      },
      "ConferenceRequestStatus": {
        "id": "ConferenceRequestStatus",
        "type": "object",
        "properties": {
          "statusCode": {
            "type": "string",
            "description": "The current status of the conference create request. Read-only.\nThe possible values are:  \n- \"pending\": the conference create request is still being processed.\n- \"success\": the conference create request succeeded, the entry points are populated.\n- \"failure\": the conference create request failed, there are no entry points."
          }
        }
      },
      "ConferenceSolution": {
        "id": "ConferenceSolution",
        "type": "object",
        "properties": {
          "iconUri": {
            "type": "string",
            "description": "The user-visible icon for this solution."
          },
          "key": {
            "$ref": "ConferenceSolutionKey",
            "description": "The key which can uniquely identify the conference solution for this event."
          },
          "name": {
            "type": "string",
            "description": "The user-visible name of this solution. Not localized."
          }
        }
      },
      "ConferenceSolutionKey": {
        "id": "ConferenceSolutionKey",
        "type": "object",
        "properties": {
          "type": {
            "type": "string",
            "description": "The conference solution type.\nIf a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.\nThe possible values are:  \n- \"eventHangout\" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)\n- \"eventNamedHangout\" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)\n- \"hangoutsMeet\" for Google Meet (http://meet.google.com)\n- \"addOn\" for 3P conference providers"
          }
        }
      },
      "CreateConferenceRequest": {
        "id": "CreateConferenceRequest",
        "type": "object",
        "properties": {
          "conferenceSolutionKey": {
            "$ref": "ConferenceSolutionKey",
            "description": "The conference solution, such as Hangouts or Google Meet."
          },
          "requestId": {
            "type": "string",
            "description": "The client-generated unique ID for this request.\nClients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored."
          },
          "status": {
            "$ref": "ConferenceRequestStatus",
            "description": "The status of the conference create request."
          }
        }
      },
      "EntryPoint": {
        "id": "EntryPoint",
        "type": "object",
        "properties": {
          "accessCode": {
            "type": "string",
            "description": "The access code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."
          },
          "entryPointFeatures": {
            "type": "array",
            "description": "Features of the entry point, such as being toll or toll-free. One entry point can have multiple features. However, toll and toll-free cannot be both set on the same entry point.",
            "items": {
              "type": "string"
            }
          },
          "entryPointType": {
            "type": "string",
            "description": "The type of the conference entry point.\nPossible values are:  \n- \"video\" - joining a conference over HTTP. A conference can have zero or one video entry point.\n- \"phone\" - joining a conference by dialing a phone number. A conference can have zero or more phone entry points.\n- \"sip\" - joining a conference over SIP. A conference can have zero or one sip entry point.\n- \"more\" - further conference joining instructions, for example additional phone numbers. A conference can have zero or one more entry point. A conference with only a more entry point is not a valid conference."
          },
          "label": {
            "type": "string",
            "description": "The label for the URI. Visible to end users. Not localized. The maximum length is 512 characters.\nExamples:  \n- for video: meet.google.com/aaa-bbbb-ccc\n- for phone: +1 123 268 2601\n- for sip: 12345678@altostrat.com\n- for more: should not be filled  \nOptional."
          },
          "meetingCode": {
            "type": "string",
            "description": "The meeting code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."
          },
          "passcode": {
            "type": "string",
            "description": "The passcode to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed."
          },
          "password": {
            "type": "string",
            "description": "The password to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."
          },
          "pin": {
            "type": "string",
            "description": "The PIN to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."
          },
          "regionCode": {
            "type": "string",
            "description": "The CLDR/ISO 3166 region code for the country associated with this phone access. Example: \"SE\" for Sweden.\nCalendar backend will populate this field only for EntryPointType.PHONE."
          },
          "uri": {
            "type": "string",
            "description": "The URI of the entry point. The maximum length is 1300 characters.\nFormat:  \n- for video, http: or https: schema is required.\n- for phone, tel: schema is required. The URI should include the entire dial sequence (e.g., tel:+12345678900,,,123456789;1234).\n- for sip, sip: schema is required, e.g., sip:12345678@myprovider.com.\n- for more, http: or https: schema is required."
          }
        }
      },
      "Error": {
        "id": "Error",
        "type": "object",
        "properties": {
          "domain": {
            "type": "string",
            "description": "Domain, or broad category, of the error."
          },
          "reason": {
            "type": "string",
            "description": "Specific reason for the error. Some of the possible values are:  \n- \"groupTooBig\" - The group of users requested is too large for a single query. \n- \"tooManyCalendarsRequested\" - The number of calendars requested is too large for a single query. \n- \"notFound\" - The requested resource was not found. \n- \"internalError\" - The API service has encountered an internal error.  Additional error types may be added in the future, so clients should gracefully handle additional error statuses not included in this list."
          }
        }
      },
      "Event": {
        "id": "Event",
        "type": "object",
        "properties": {
          "anyoneCanAddSelf": {
            "type": "boolean",
            "description": "Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.",
            "default": "false"
          },
          "attachments": {
            "type": "array",
            "description": "File attachments for the event.\nIn order to modify attachments the supportsAttachments request parameter should be set to true.\nThere can be at most 25 attachments per event,",
            "items": {
              "$ref": "EventAttachment"
            }
          },
          "attendees": {
            "type": "array",
            "description": "The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.",
            "items": {
              "$ref": "EventAttendee"
            }
          },
          "attendeesOmitted": {
            "type": "boolean",
            "description": "Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.",
            "default": "false"
          },
          "colorId": {
            "type": "string",
            "description": "The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional."
          },
          "conferenceData": {
            "$ref": "ConferenceData",
            "description": "The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests."
          },
          "created": {
            "type": "string",
            "description": "Creation time of the event (as a RFC3339 timestamp). Read-only.",
            "format": "date-time"
          },
          "creator": {
            "type": "object",
            "description": "The creator of the event. Read-only.",
            "properties": {
              "displayName": {
                "type": "string",
                "description": "The creator's name, if available."
              },
              "email": {
                "type": "string",
                "description": "The creator's email address, if available."
              },
              "id": {
                "type": "string",
                "description": "The creator's Profile ID, if available."
              },
              "self": {
                "type": "boolean",
                "description": "Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.",
                "default": "false"
              }
            }
          },
          "description": {
            "type": "string",
            "description": "Description of the event. Can contain HTML. Optional."
          },
          "end": {
            "$ref": "EventDateTime",
            "description": "The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance.",
            "annotations": {
              "required": [
                "calendar.events.import",
                "calendar.events.insert",
                "calendar.events.update"
              ]
            }
          },
          "endTimeUnspecified": {
            "type": "boolean",
            "description": "Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.",
            "default": "false"
          },
          "etag": {
            "type": "string",
            "description": "ETag of the resource."
          },
          "eventType": {
            "type": "string",
            "description": "Specific type of the event. This cannot be modified after the event is created. Possible values are:  \n- \"default\" - A regular event or not further specified. \n- \"outOfOffice\" - An out-of-office event. \n- \"focusTime\" - A focus-time event. \n- \"workingLocation\" - A working location event. \n- \"fromGmail\" - An event from Gmail. This type of event cannot be created.",
            "default": "default"
          },
          "extendedProperties": {
            "type": "object",
            "description": "Extended properties of the event.",
            "properties": {
              "private": {
                "type": "object",
                "description": "Properties that are private to the copy of the event that appears on this calendar.",
                "additionalProperties": {
                  "type": "string",
                  "description": "The name of the private property and the corresponding value."
                }
              },
              "shared": {
                "type": "object",
                "description": "Properties that are shared between copies of the event on other attendees' calendars.",
                "additionalProperties": {
                  "type": "string",
                  "description": "The name of the shared property and the corresponding value."
                }
              }
            }
          },
          "focusTimeProperties": {
            "$ref": "EventFocusTimeProperties",
            "description": "Focus Time event data. Used if eventType is focusTime."
          },
          "gadget": {
            "type": "object",
            "description": "A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.",
            "properties": {
              "display": {
                "type": "string",
                "description": "The gadget's display mode. Deprecated. Possible values are:  \n- \"icon\" - The gadget displays next to the event's title in the calendar view. \n- \"chip\" - The gadget displays when the event is clicked."
              },
              "height": {
                "type": "integer",
                "description": "The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.",
                "format": "int32"
              },
              "iconLink": {
                "type": "string",
                "description": "The gadget's icon URL. The URL scheme must be HTTPS. Deprecated."
              },
              "link": {
                "type": "string",
                "description": "The gadget's URL. The URL scheme must be HTTPS. Deprecated."
              },
              "preferences": {
                "type": "object",
                "description": "Preferences.",
                "additionalProperties": {
                  "type": "string",
                  "description": "The preference name and corresponding value."
                }
              },
              "title": {
                "type": "string",
                "description": "The gadget's title. Deprecated."
              },
              "type": {
                "type": "string",
                "description": "The gadget's type. Deprecated."
              },
              "width": {
                "type": "integer",
                "description": "The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.",
                "format": "int32"
              }
            }
          },
          "guestsCanInviteOthers": {
            "type": "boolean",
            "description": "Whether attendees other than the organizer can invite others to the event. Optional. The default is True.",
            "default": "true"
          },
          "guestsCanModify": {
            "type": "boolean",
            "description": "Whether attendees other than the organizer can modify the event. Optional. The default is False.",
            "default": "false"
          },
          "guestsCanSeeOtherGuests": {
            "type": "boolean",
            "description": "Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.",
            "default": "true"
          },
          "hangoutLink": {
            "type": "string",
            "description": "An absolute link to the Google Hangout associated with this event. Read-only."
          },
          "htmlLink": {
            "type": "string",
            "description": "An absolute link to this event in the Google Calendar Web UI. Read-only."
          },
          "iCalUID": {
            "type": "string",
            "description": "Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.\nNote that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.",
            "annotations": {
              "required": [
                "calendar.events.import"
              ]
            }
          },
          "id": {
            "type": "string",
            "description": "Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  \n- characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 \n- the length of the ID must be between 5 and 1024 characters \n- the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.\nIf you do not specify an ID, it will be automatically generated by the server.\nNote that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs."
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#event\").",
            "default": "calendar#event"
          },
          "location": {
            "type": "string",
            "description": "Geographic location of the event as free-form text. Optional."
          },
          "locked": {
            "type": "boolean",
            "description": "Whether this is a locked event copy where no changes can be made to the main event fields \"summary\", \"description\", \"location\", \"start\", \"end\" or \"recurrence\". The default is False. Read-Only.",
            "default": "false"
          },
          "organizer": {
            "type": "object",
            "description": "The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.",
            "properties": {
              "displayName": {
                "type": "string",
                "description": "The organizer's name, if available."
              },
              "email": {
                "type": "string",
                "description": "The organizer's email address, if available. It must be a valid email address as per RFC5322."
              },
              "id": {
                "type": "string",
                "description": "The organizer's Profile ID, if available."
              },
              "self": {
                "type": "boolean",
                "description": "Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.",
                "default": "false"
              }
            }
          },
          "originalStartTime": {
            "$ref": "EventDateTime",
            "description": "For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable."
          },
          "outOfOfficeProperties": {
            "$ref": "EventOutOfOfficeProperties",
            "description": "Out of office event data. Used if eventType is outOfOffice."
          },
          "privateCopy": {
            "type": "boolean",
            "description": "If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.",
            "default": "false"
          },
          "recurrence": {
            "type": "array",
            "description": "List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.",
            "items": {
              "type": "string"
            }
          },
          "recurringEventId": {
            "type": "string",
            "description": "For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable."
          },
          "reminders": {
            "type": "object",
            "description": "Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event.",
            "properties": {
              "overrides": {
                "type": "array",
                "description": "If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.",
                "items": {
                  "$ref": "EventReminder"
                }
              },
              "useDefault": {
                "type": "boolean",
                "description": "Whether the default reminders of the calendar apply to the event."
              }
            }
          },
          "sequence": {
            "type": "integer",
            "description": "Sequence number as per iCalendar.",
            "format": "int32"
          },
          "source": {
            "type": "object",
            "description": "Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.",
            "properties": {
              "title": {
                "type": "string",
                "description": "Title of the source; for example a title of a web page or an email subject."
              },
              "url": {
                "type": "string",
                "description": "URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS."
              }
            }
          },
          "start": {
            "$ref": "EventDateTime",
            "description": "The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance.",
            "annotations": {
              "required": [
                "calendar.events.import",
                "calendar.events.insert",
                "calendar.events.update"
              ]
            }
          },
          "status": {
            "type": "string",
            "description": "Status of the event. Optional. Possible values are:  \n- \"confirmed\" - The event is confirmed. This is the default status. \n- \"tentative\" - The event is tentatively confirmed. \n- \"cancelled\" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.\nA cancelled status represents two different states depending on the event type:  \n- Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.\nCancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  \n- All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.\nDeleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.\nIf an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated."
          },
          "summary": {
            "type": "string",
            "description": "Title of the event."
          },
          "transparency": {
            "type": "string",
            "description": "Whether the event blocks time on the calendar. Optional. Possible values are:  \n- \"opaque\" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. \n- \"transparent\" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.",
            "default": "opaque"
          },
          "updated": {
            "type": "string",
            "description": "Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only.",
            "format": "date-time"
          },
          "visibility": {
            "type": "string",
            "description": "Visibility of the event. Optional. Possible values are:  \n- \"default\" - Uses the default visibility for events on the calendar. This is the default value. \n- \"public\" - The event is public and event details are visible to all readers of the calendar. \n- \"private\" - The event is private and only event attendees may view event details. \n- \"confidential\" - The event is private. This value is provided for compatibility reasons.",
            "default": "default"
          },
          "workingLocationProperties": {
            "$ref": "EventWorkingLocationProperties",
            "description": "Working location event data."
          }
        }
      },
      "EventAttachment": {
        "id": "EventAttachment",
        "type": "object",
        "properties": {
          "fileId": {
            "type": "string",
            "description": "ID of the attached file. Read-only.\nFor Google Drive files, this is the ID of the corresponding Files resource entry in the Drive API."
          },
          "fileUrl": {
            "type": "string",
            "description": "URL link to the attachment.\nFor adding Google Drive file attachments use the same format as in alternateLink property of the Files resource in the Drive API.\nRequired when adding an attachment."
          },
          "iconLink": {
            "type": "string",
            "description": "URL link to the attachment's icon. This field can only be modified for custom third-party attachments."
          },
          "mimeType": {
            "type": "string",
            "description": "Internet media type (MIME type) of the attachment."
          },
          "title": {
            "type": "string",
            "description": "Attachment title."
          }
        }
      },
      "EventAttendee": {
        "id": "EventAttendee",
        "type": "object",
        "properties": {
          "additionalGuests": {
            "type": "integer",
            "description": "Number of additional guests. Optional. The default is 0.",
            "default": "0",
            "format": "int32"
          },
          "comment": {
            "type": "string",
            "description": "The attendee's response comment. Optional."
          },
          "displayName": {
            "type": "string",
            "description": "The attendee's name, if available. Optional."
          },
          "email": {
            "type": "string",
            "description": "The attendee's email address, if available. This field must be present when adding an attendee. It must be a valid email address as per RFC5322.\nRequired when adding an attendee."
          },
          "id": {
            "type": "string",
            "description": "The attendee's Profile ID, if available."
          },
          "optional": {
            "type": "boolean",
            "description": "Whether this is an optional attendee. Optional. The default is False.",
            "default": "false"
          },
          "organizer": {
            "type": "boolean",
            "description": "Whether the attendee is the organizer of the event. Read-only. The default is False."
          },
          "resource": {
            "type": "boolean",
            "description": "Whether the attendee is a resource. Can only be set when the attendee is added to the event for the first time. Subsequent modifications are ignored. Optional. The default is False.",
            "default": "false"
          },
          "responseStatus": {
            "type": "string",
            "description": "The attendee's response status. Possible values are:  \n- \"needsAction\" - The attendee has not responded to the invitation (recommended for new events). \n- \"declined\" - The attendee has declined the invitation. \n- \"tentative\" - The attendee has tentatively accepted the invitation. \n- \"accepted\" - The attendee has accepted the invitation.  Warning: If you add an event using the values declined, tentative, or accepted, attendees with the \"Add invitations to my calendar\" setting set to \"When I respond to invitation in email\" won't see an event on their calendar unless they choose to change their invitation response in the event invitation email."
          },
          "self": {
            "type": "boolean",
            "description": "Whether this entry represents the calendar on which this copy of the event appears. Read-only. The default is False.",
            "default": "false"
          }
        }
      },
      "EventDateTime": {
        "id": "EventDateTime",
        "type": "object",
        "properties": {
          "date": {
            "type": "string",
            "description": "The date, in the format \"yyyy-mm-dd\", if this is an all-day event.",
            "format": "date"
          },
          "dateTime": {
            "type": "string",
            "description": "The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.",
            "format": "date-time"
          },
          "timeZone": {
            "type": "string",
            "description": "The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end."
          }
        }
      },
      "EventFocusTimeProperties": {
        "id": "EventFocusTimeProperties",
        "type": "object",
        "properties": {
          "autoDeclineMode": {
            "type": "string",
            "description": "Whether to decline meeting invitations which overlap Focus Time events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Focus Time event is present are to be declined."
          },
          "chatStatus": {
            "type": "string",
            "description": "The status to mark the user in Chat and related products. This can be available or doNotDisturb."
          },
          "declineMessage": {
            "type": "string",
            "description": "Response message to set if an existing event or new invitation is automatically declined by Calendar."
          }
        }
      },
      "EventOutOfOfficeProperties": {
        "id": "EventOutOfOfficeProperties",
        "type": "object",
        "properties": {
          "autoDeclineMode": {
            "type": "string",
            "description": "Whether to decline meeting invitations which overlap Out of office events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Out of office event is present are to be declined."
          },
          "declineMessage": {
            "type": "string",
            "description": "Response message to set if an existing event or new invitation is automatically declined by Calendar."
          }
        }
      },
      "EventReminder": {
        "id": "EventReminder",
        "type": "object",
        "properties": {
          "method": {
            "type": "string",
            "description": "The method used by this reminder. Possible values are:  \n- \"email\" - Reminders are sent via email. \n- \"popup\" - Reminders are sent via a UI popup.  \nRequired when adding a reminder."
          },
          "minutes": {
            "type": "integer",
            "description": "Number of minutes before the start of the event when the reminder should trigger. Valid values are between 0 and 40320 (4 weeks in minutes).\nRequired when adding a reminder.",
            "format": "int32"
          }
        }
      },
      "EventWorkingLocationProperties": {
        "id": "EventWorkingLocationProperties",
        "type": "object",
        "properties": {
          "customLocation": {
            "type": "object",
            "description": "If present, specifies that the user is working from a custom location.",
            "properties": {
              "label": {
                "type": "string",
                "description": "An optional extra label for additional information."
              }
            }
          },
          "homeOffice": {
            "type": "any",
            "description": "If present, specifies that the user is working at home."
          },
          "officeLocation": {
            "type": "object",
            "description": "If present, specifies that the user is working from an office.",
            "properties": {
              "buildingId": {
                "type": "string",
                "description": "An optional building identifier. This should reference a building ID in the organization's Resources database."
              },
              "deskId": {
                "type": "string",
                "description": "An optional desk identifier."
              },
              "floorId": {
                "type": "string",
                "description": "An optional floor identifier."
              },
              "floorSectionId": {
                "type": "string",
                "description": "An optional floor section identifier."
              },
              "label": {
                "type": "string",
                "description": "The office name that's displayed in Calendar Web and Mobile clients. We recommend you reference a building name in the organization's Resources database."
              }
            }
          },
          "type": {
            "type": "string",
            "description": "Type of the working location. Possible values are:  \n- \"homeOffice\" - The user is working at home. \n- \"officeLocation\" - The user is working from an office. \n- \"customLocation\" - The user is working from a custom location.  Any details are specified in a sub-field of the specified name, but this field may be missing if empty. Any other fields are ignored.\nRequired when adding working location properties."
          }
        }
      },
      "Events": {
        "id": "Events",
        "type": "object",
        "properties": {
          "accessRole": {
            "type": "string",
            "description": "The user's access role for this calendar. Read-only. Possible values are:  \n- \"none\" - The user has no access. \n- \"freeBusyReader\" - The user has read access to free/busy information. \n- \"reader\" - The user has read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - The user has read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - The user has ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."
          },
          "defaultReminders": {
            "type": "array",
            "description": "The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True).",
            "items": {
              "$ref": "EventReminder"
            }
          },
          "description": {
            "type": "string",
            "description": "Description of the calendar. Read-only."
          },
          "etag": {
            "type": "string",
            "description": "ETag of the collection."
          },
          "items": {
            "type": "array",
            "description": "List of events on the calendar.",
            "items": {
              "$ref": "Event"
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the collection (\"calendar#events\").",
            "default": "calendar#events"
          },
          "nextPageToken": {
            "type": "string",
            "description": "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."
          },
          "nextSyncToken": {
            "type": "string",
            "description": "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."
          },
          "summary": {
            "type": "string",
            "description": "Title of the calendar. Read-only."
          },
          "timeZone": {
            "type": "string",
            "description": "The time zone of the calendar. Read-only."
          },
          "updated": {
            "type": "string",
            "description": "Last modification time of the calendar (as a RFC3339 timestamp). Read-only.",
            "format": "date-time"
          }
        }
      },
      "FreeBusyCalendar": {
        "id": "FreeBusyCalendar",
        "type": "object",
        "properties": {
          "busy": {
            "type": "array",
            "description": "List of time ranges during which this calendar should be regarded as busy.",
            "items": {
              "$ref": "TimePeriod"
            }
          },
          "errors": {
            "type": "array",
            "description": "Optional error(s) (if computation for the calendar failed).",
            "items": {
              "$ref": "Error"
            }
          }
        }
      },
      "FreeBusyGroup": {
        "id": "FreeBusyGroup",
        "type": "object",
        "properties": {
          "calendars": {
            "type": "array",
            "description": "List of calendars' identifiers within a group.",
            "items": {
              "type": "string"
            }
          },
          "errors": {
            "type": "array",
            "description": "Optional error(s) (if computation for the group failed).",
            "items": {
              "$ref": "Error"
            }
          }
        }
      },
      "FreeBusyRequest": {
        "id": "FreeBusyRequest",
        "type": "object",
        "properties": {
          "calendarExpansionMax": {
            "type": "integer",
            "description": "Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50.",
            "format": "int32"
          },
          "groupExpansionMax": {
            "type": "integer",
            "description": "Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100.",
            "format": "int32"
          },
          "items": {
            "type": "array",
            "description": "List of calendars and/or groups to query.",
            "items": {
              "$ref": "FreeBusyRequestItem"
            }
          },
          "timeMax": {
            "type": "string",
            "description": "The end of the interval for the query formatted as per RFC3339.",
            "format": "date-time"
          },
          "timeMin": {
            "type": "string",
            "description": "The start of the interval for the query formatted as per RFC3339.",
            "format": "date-time"
          },
          "timeZone": {
            "type": "string",
            "description": "Time zone used in the response. Optional. The default is UTC.",
            "default": "UTC"
          }
        }
      },
      "FreeBusyRequestItem": {
        "id": "FreeBusyRequestItem",
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The identifier of a calendar or a group."
          }
        }
      },
      "FreeBusyResponse": {
        "id": "FreeBusyResponse",
        "type": "object",
        "properties": {
          "calendars": {
            "type": "object",
            "description": "List of free/busy information for calendars.",
            "additionalProperties": {
              "$ref": "FreeBusyCalendar",
              "description": "Free/busy expansions for a single calendar."
            }
          },
          "groups": {
            "type": "object",
            "description": "Expansion of groups.",
            "additionalProperties": {
              "$ref": "FreeBusyGroup",
              "description": "List of calendars that are members of this group."
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#freeBusy\").",
            "default": "calendar#freeBusy"
          },
          "timeMax": {
            "type": "string",
            "description": "The end of the interval.",
            "format": "date-time"
          },
          "timeMin": {
            "type": "string",
            "description": "The start of the interval.",
            "format": "date-time"
          }
        }
      },
      "Setting": {
        "id": "Setting",
        "type": "object",
        "properties": {
          "etag": {
            "type": "string",
            "description": "ETag of the resource."
          },
          "id": {
            "type": "string",
            "description": "The id of the user setting."
          },
          "kind": {
            "type": "string",
            "description": "Type of the resource (\"calendar#setting\").",
            "default": "calendar#setting"
          },
          "value": {
            "type": "string",
            "description": "Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters."
          }
        }
      },
      "Settings": {
        "id": "Settings",
        "type": "object",
        "properties": {
          "etag": {
            "type": "string",
            "description": "Etag of the collection."
          },
          "items": {
            "type": "array",
            "description": "List of user settings.",
            "items": {
              "$ref": "Setting"
            }
          },
          "kind": {
            "type": "string",
            "description": "Type of the collection (\"calendar#settings\").",
            "default": "calendar#settings"
          },
          "nextPageToken": {
            "type": "string",
            "description": "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."
          },
          "nextSyncToken": {
            "type": "string",
            "description": "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."
          }
        }
      },
      "TimePeriod": {
        "id": "TimePeriod",
        "type": "object",
        "properties": {
          "end": {
            "type": "string",
            "description": "The (exclusive) end of the time period.",
            "format": "date-time"
          },
          "start": {
            "type": "string",
            "description": "The (inclusive) start of the time period.",
            "format": "date-time"
          }
        }
      }
    },
    "ownerDomain": "google.com",
    "basePath": "/calendar/v3/",
    "version": "v3",
    "title": "Calendar API",
    "parameters": {
      "alt": {
        "type": "string",
        "description": "Data format for the response.",
        "default": "json",
        "enum": [
          "json"
        ],
        "enumDescriptions": [
          "Responses with Content-Type of application/json"
        ],
        "location": "query"
      },
      "fields": {
        "type": "string",
        "description": "Selector specifying which fields to include in a partial response.",
        "location": "query"
      },
      "key": {
        "type": "string",
        "description": "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.",
        "location": "query"
      },
      "oauth_token": {
        "type": "string",
        "description": "OAuth 2.0 token for the current user.",
        "location": "query"
      },
      "prettyPrint": {
        "type": "boolean",
        "description": "Returns response with indentations and line breaks.",
        "default": "true",
        "location": "query"
      },
      "quotaUser": {
        "type": "string",
        "description": "An opaque string that represents a user for quota purposes. Must not exceed 40 characters.",
        "location": "query"
      },
      "userIp": {
        "type": "string",
        "description": "Deprecated. Please use quotaUser instead.",
        "location": "query"
      }
    },
    "servicePath": "calendar/v3/",
    "ownerName": "Google",
    "description": "Manipulates events and other calendar data.",
    "resources": {
      "acl": {
        "methods": {
          "delete": {
            "id": "calendar.acl.delete",
            "path": "calendars/{calendarId}/acl/{ruleId}",
            "httpMethod": "DELETE",
            "description": "Deletes an access control rule.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "ruleId": {
                "type": "string",
                "description": "ACL rule identifier.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId",
              "ruleId"
            ],
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "get": {
            "id": "calendar.acl.get",
            "path": "calendars/{calendarId}/acl/{ruleId}",
            "httpMethod": "GET",
            "description": "Returns an access control rule.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "ruleId": {
                "type": "string",
                "description": "ACL rule identifier.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId",
              "ruleId"
            ],
            "response": {
              "$ref": "AclRule"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          },
          "insert": {
            "id": "calendar.acl.insert",
            "path": "calendars/{calendarId}/acl",
            "httpMethod": "POST",
            "description": "Creates an access control rule.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Whether to send notifications about the calendar sharing change. Optional. The default is True.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "AclRule"
            },
            "response": {
              "$ref": "AclRule"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "list": {
            "id": "calendar.acl.list",
            "path": "calendars/{calendarId}/acl",
            "httpMethod": "GET",
            "description": "Returns the rules in the access control list for the calendar.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to \"none\". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "response": {
              "$ref": "Acl"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ],
            "supportsSubscription": true
          },
          "patch": {
            "id": "calendar.acl.patch",
            "path": "calendars/{calendarId}/acl/{ruleId}",
            "httpMethod": "PATCH",
            "description": "Updates an access control rule. This method supports patch semantics.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "ruleId": {
                "type": "string",
                "description": "ACL rule identifier.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "ruleId"
            ],
            "request": {
              "$ref": "AclRule"
            },
            "response": {
              "$ref": "AclRule"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "update": {
            "id": "calendar.acl.update",
            "path": "calendars/{calendarId}/acl/{ruleId}",
            "httpMethod": "PUT",
            "description": "Updates an access control rule.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "ruleId": {
                "type": "string",
                "description": "ACL rule identifier.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "ruleId"
            ],
            "request": {
              "$ref": "AclRule"
            },
            "response": {
              "$ref": "AclRule"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "watch": {
            "id": "calendar.acl.watch",
            "path": "calendars/{calendarId}/acl/watch",
            "httpMethod": "POST",
            "description": "Watch for changes to ACL resources.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to \"none\". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Channel",
              "parameterName": "resource"
            },
            "response": {
              "$ref": "Channel"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ],
            "supportsSubscription": true
          }
        }
      },
      "calendarList": {
        "methods": {
          "delete": {
            "id": "calendar.calendarList.delete",
            "path": "users/me/calendarList/{calendarId}",
            "httpMethod": "DELETE",
            "description": "Removes a calendar from the user's calendar list.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "get": {
            "id": "calendar.calendarList.get",
            "path": "users/me/calendarList/{calendarId}",
            "httpMethod": "GET",
            "description": "Returns a calendar from the user's calendar list.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "response": {
              "$ref": "CalendarListEntry"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          },
          "insert": {
            "id": "calendar.calendarList.insert",
            "path": "users/me/calendarList",
            "httpMethod": "POST",
            "description": "Inserts an existing calendar into the user's calendar list.",
            "parameters": {
              "colorRgbFormat": {
                "type": "boolean",
                "description": "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.",
                "location": "query"
              }
            },
            "request": {
              "$ref": "CalendarListEntry"
            },
            "response": {
              "$ref": "CalendarListEntry"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "list": {
            "id": "calendar.calendarList.list",
            "path": "users/me/calendarList",
            "httpMethod": "GET",
            "description": "Returns the calendars on the user's calendar list.",
            "parameters": {
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "minAccessRole": {
                "type": "string",
                "description": "The minimum access role for the user in the returned entries. Optional. The default is no restriction.",
                "enum": [
                  "freeBusyReader",
                  "owner",
                  "reader",
                  "writer"
                ],
                "enumDescriptions": [
                  "The user can read free/busy information.",
                  "The user can read and modify events and access control lists.",
                  "The user can read events that are not private.",
                  "The user can read and modify events."
                ],
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted calendar list entries in the result. Optional. The default is False.",
                "location": "query"
              },
              "showHidden": {
                "type": "boolean",
                "description": "Whether to show hidden entries. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.\nTo ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "response": {
              "$ref": "CalendarList"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ],
            "supportsSubscription": true
          },
          "patch": {
            "id": "calendar.calendarList.patch",
            "path": "users/me/calendarList/{calendarId}",
            "httpMethod": "PATCH",
            "description": "Updates an existing calendar on the user's calendar list. This method supports patch semantics.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "colorRgbFormat": {
                "type": "boolean",
                "description": "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "CalendarListEntry"
            },
            "response": {
              "$ref": "CalendarListEntry"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "update": {
            "id": "calendar.calendarList.update",
            "path": "users/me/calendarList/{calendarId}",
            "httpMethod": "PUT",
            "description": "Updates an existing calendar on the user's calendar list.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "colorRgbFormat": {
                "type": "boolean",
                "description": "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "CalendarListEntry"
            },
            "response": {
              "$ref": "CalendarListEntry"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "watch": {
            "id": "calendar.calendarList.watch",
            "path": "users/me/calendarList/watch",
            "httpMethod": "POST",
            "description": "Watch for changes to CalendarList resources.",
            "parameters": {
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "minAccessRole": {
                "type": "string",
                "description": "The minimum access role for the user in the returned entries. Optional. The default is no restriction.",
                "enum": [
                  "freeBusyReader",
                  "owner",
                  "reader",
                  "writer"
                ],
                "enumDescriptions": [
                  "The user can read free/busy information.",
                  "The user can read and modify events and access control lists.",
                  "The user can read events that are not private.",
                  "The user can read and modify events."
                ],
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted calendar list entries in the result. Optional. The default is False.",
                "location": "query"
              },
              "showHidden": {
                "type": "boolean",
                "description": "Whether to show hidden entries. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.\nTo ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "request": {
              "$ref": "Channel",
              "parameterName": "resource"
            },
            "response": {
              "$ref": "Channel"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ],
            "supportsSubscription": true
          }
        }
      },
      "calendars": {
        "methods": {
          "clear": {
            "id": "calendar.calendars.clear",
            "path": "calendars/{calendarId}/clear",
            "httpMethod": "POST",
            "description": "Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "delete": {
            "id": "calendar.calendars.delete",
            "path": "calendars/{calendarId}",
            "httpMethod": "DELETE",
            "description": "Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "get": {
            "id": "calendar.calendars.get",
            "path": "calendars/{calendarId}",
            "httpMethod": "GET",
            "description": "Returns metadata for a calendar.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "response": {
              "$ref": "Calendar"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          },
          "insert": {
            "id": "calendar.calendars.insert",
            "path": "calendars",
            "httpMethod": "POST",
            "description": "Creates a secondary calendar.",
            "request": {
              "$ref": "Calendar"
            },
            "response": {
              "$ref": "Calendar"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "patch": {
            "id": "calendar.calendars.patch",
            "path": "calendars/{calendarId}",
            "httpMethod": "PATCH",
            "description": "Updates metadata for a calendar. This method supports patch semantics.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Calendar"
            },
            "response": {
              "$ref": "Calendar"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          },
          "update": {
            "id": "calendar.calendars.update",
            "path": "calendars/{calendarId}",
            "httpMethod": "PUT",
            "description": "Updates metadata for a calendar.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Calendar"
            },
            "response": {
              "$ref": "Calendar"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar"
            ]
          }
        }
      },
      "channels": {
        "methods": {
          "stop": {
            "id": "calendar.channels.stop",
            "path": "channels/stop",
            "httpMethod": "POST",
            "description": "Stop watching resources through this channel",
            "request": {
              "$ref": "Channel",
              "parameterName": "resource"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events",
              "https://www.googleapis.com/auth/calendar.events.readonly",
              "https://www.googleapis.com/auth/calendar.readonly",
              "https://www.googleapis.com/auth/calendar.settings.readonly"
            ]
          }
        }
      },
      "colors": {
        "methods": {
          "get": {
            "id": "calendar.colors.get",
            "path": "colors",
            "httpMethod": "GET",
            "description": "Returns the color definitions for calendars and events.",
            "response": {
              "$ref": "Colors"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          }
        }
      },
      "events": {
        "methods": {
          "delete": {
            "id": "calendar.events.delete",
            "path": "calendars/{calendarId}/events/{eventId}",
            "httpMethod": "DELETE",
            "description": "Deletes an event.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "eventId": {
                "type": "string",
                "description": "Event identifier.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the deletion of the event. Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Guests who should receive notifications about the deletion of the event.",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. For calendar migration tasks, consider using the Events.import method instead."
                ],
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId"
            ],
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "get": {
            "id": "calendar.events.get",
            "path": "calendars/{calendarId}/events/{eventId}",
            "httpMethod": "GET",
            "description": "Returns an event based on its Google Calendar ID. To retrieve an event using its iCalendar ID, call the events.list method using the iCalUID parameter.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "eventId": {
                "type": "string",
                "description": "Event identifier.",
                "required": true,
                "location": "path"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "timeZone": {
                "type": "string",
                "description": "Time zone used in the response. Optional. The default is the time zone of the calendar.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId"
            ],
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events",
              "https://www.googleapis.com/auth/calendar.events.readonly",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          },
          "import": {
            "id": "calendar.events.import",
            "path": "calendars/{calendarId}/events/import",
            "httpMethod": "POST",
            "description": "Imports an event. This operation is used to add a private copy of an existing event to a calendar. Only events with an eventType of default may be imported.\nDeprecated behavior: If a non-default event is imported, its type will be changed to default and any event-type-specific properties it may have will be dropped.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "conferenceDataVersion": {
                "type": "integer",
                "description": "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.",
                "format": "int32",
                "minimum": "0",
                "maximum": "1",
                "location": "query"
              },
              "supportsAttachments": {
                "type": "boolean",
                "description": "Whether API client performing operation supports event attachments. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Event"
            },
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "insert": {
            "id": "calendar.events.insert",
            "path": "calendars/{calendarId}/events",
            "httpMethod": "POST",
            "description": "Creates an event.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "conferenceDataVersion": {
                "type": "integer",
                "description": "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.",
                "format": "int32",
                "minimum": "0",
                "maximum": "1",
                "location": "query"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the creation of the new event. Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. Warning: Using the value none can have significant adverse effects, including events not syncing to external calendars or events being lost altogether for some users. For calendar migration tasks, consider using the events.import method instead."
                ],
                "location": "query"
              },
              "supportsAttachments": {
                "type": "boolean",
                "description": "Whether API client performing operation supports event attachments. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Event"
            },
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "instances": {
            "id": "calendar.events.instances",
            "path": "calendars/{calendarId}/events/{eventId}/instances",
            "httpMethod": "GET",
            "description": "Returns instances of the specified recurring event.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "eventId": {
                "type": "string",
                "description": "Recurring event identifier.",
                "required": true,
                "location": "path"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of events returned on one result page. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "originalStart": {
                "type": "string",
                "description": "The original start time of the instance in the result. Optional.",
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events will still be included if singleEvents is False. Optional. The default is False.",
                "location": "query"
              },
              "timeMax": {
                "type": "string",
                "description": "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset.",
                "format": "date-time",
                "location": "query"
              },
              "timeMin": {
                "type": "string",
                "description": "Lower bound (inclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset.",
                "format": "date-time",
                "location": "query"
              },
              "timeZone": {
                "type": "string",
                "description": "Time zone used in the response. Optional. The default is the time zone of the calendar.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId"
            ],
            "response": {
              "$ref": "Events"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events",
              "https://www.googleapis.com/auth/calendar.events.readonly",
              "https://www.googleapis.com/auth/calendar.readonly"
            ],
            "supportsSubscription": true
          },
          "list": {
            "id": "calendar.events.list",
            "path": "calendars/{calendarId}/events",
            "httpMethod": "GET",
            "description": "Returns events on the specified calendar.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored.",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "eventTypes": {
                "type": "string",
                "description": "Event types to return. Optional. This parameter can be repeated multiple times to return events of different types. If unset, returns all event types.",
                "enum": [
                  "default",
                  "focusTime",
                  "fromGmail",
                  "outOfOffice",
                  "workingLocation"
                ],
                "enumDescriptions": [
                  "Regular events.",
                  "Focus time events.",
                  "Events from Gmail.",
                  "Out of office events.",
                  "Working location events."
                ],
                "repeated": true,
                "location": "query"
              },
              "iCalUID": {
                "type": "string",
                "description": "Specifies an event ID in the iCalendar format to be provided in the response. Optional. Use this if you want to search for an event by its iCalendar ID.",
                "location": "query"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.",
                "default": "250",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "orderBy": {
                "type": "string",
                "description": "The order of the events returned in the result. Optional. The default is an unspecified, stable order.",
                "enum": [
                  "startTime",
                  "updated"
                ],
                "enumDescriptions": [
                  "Order by the start date/time (ascending). This is only available when querying single events (i.e. the parameter singleEvents is True)",
                  "Order by last modification time (ascending)."
                ],
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "privateExtendedProperty": {
                "type": "string",
                "description": "Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.",
                "repeated": true,
                "location": "query"
              },
              "q": {
                "type": "string",
                "description": "Free text search terms to find events that match these terms in the following fields:\n\n- summary \n- description \n- location \n- attendee's displayName \n- attendee's email \n- organizer's displayName \n- organizer's email \n- workingLocationProperties.officeLocation.buildingId \n- workingLocationProperties.officeLocation.deskId \n- workingLocationProperties.officeLocation.label \n- workingLocationProperties.customLocation.label \nThese search terms also match predefined keywords against all display title translations of working location, out-of-office, and focus-time events. For example, searching for \"Office\" or \"Bureau\" returns working location events of type officeLocation, whereas searching for \"Out of office\" or \"Abwesend\" returns out-of-office events. Optional.",
                "location": "query"
              },
              "sharedExtendedProperty": {
                "type": "string",
                "description": "Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.",
                "repeated": true,
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.",
                "location": "query"
              },
              "showHiddenInvitations": {
                "type": "boolean",
                "description": "Whether to include hidden invitations in the result. Optional. The default is False.",
                "location": "query"
              },
              "singleEvents": {
                "type": "boolean",
                "description": "Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nThere are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.\n\nThese are: \n- iCalUID \n- orderBy \n- privateExtendedProperty \n- q \n- sharedExtendedProperty \n- timeMin \n- timeMax \n- updatedMin All other query parameters should be the same as for the initial synchronization to avoid undefined behavior. If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              },
              "timeMax": {
                "type": "string",
                "description": "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.",
                "format": "date-time",
                "location": "query"
              },
              "timeMin": {
                "type": "string",
                "description": "Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.",
                "format": "date-time",
                "location": "query"
              },
              "timeZone": {
                "type": "string",
                "description": "Time zone used in the response. Optional. The default is the time zone of the calendar.",
                "location": "query"
              },
              "updatedMin": {
                "type": "string",
                "description": "Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.",
                "format": "date-time",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "response": {
              "$ref": "Events"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events",
              "https://www.googleapis.com/auth/calendar.events.readonly",
              "https://www.googleapis.com/auth/calendar.readonly"
            ],
            "supportsSubscription": true
          },
          "move": {
            "id": "calendar.events.move",
            "path": "calendars/{calendarId}/events/{eventId}/move",
            "httpMethod": "POST",
            "description": "Moves an event to another calendar, i.e. changes an event's organizer. Note that only default events can be moved; outOfOffice, focusTime, workingLocation and fromGmail events cannot be moved.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier of the source calendar where the event currently is on.",
                "required": true,
                "location": "path"
              },
              "destination": {
                "type": "string",
                "description": "Calendar identifier of the target calendar where the event is to be moved to.",
                "required": true,
                "location": "query"
              },
              "eventId": {
                "type": "string",
                "description": "Event identifier.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the change of the event's organizer. Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Guests who should receive notifications about the change of the event's organizer.",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. For calendar migration tasks, consider using the Events.import method instead."
                ],
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId",
              "destination"
            ],
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "patch": {
            "id": "calendar.events.patch",
            "path": "calendars/{calendarId}/events/{eventId}",
            "httpMethod": "PATCH",
            "description": "Updates an event. This method supports patch semantics.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "conferenceDataVersion": {
                "type": "integer",
                "description": "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.",
                "format": "int32",
                "minimum": "0",
                "maximum": "1",
                "location": "query"
              },
              "eventId": {
                "type": "string",
                "description": "Event identifier.",
                "required": true,
                "location": "path"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Guests who should receive notifications about the event update (for example, title changes, etc.).",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. For calendar migration tasks, consider using the Events.import method instead."
                ],
                "location": "query"
              },
              "supportsAttachments": {
                "type": "boolean",
                "description": "Whether API client performing operation supports event attachments. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId"
            ],
            "request": {
              "$ref": "Event"
            },
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "quickAdd": {
            "id": "calendar.events.quickAdd",
            "path": "calendars/{calendarId}/events/quickAdd",
            "httpMethod": "POST",
            "description": "Creates an event based on a simple text string.",
            "parameters": {
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the creation of the event. Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Guests who should receive notifications about the creation of the new event.",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. For calendar migration tasks, consider using the Events.import method instead."
                ],
                "location": "query"
              },
              "text": {
                "type": "string",
                "description": "The text describing the event to be created.",
                "required": true,
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "text"
            ],
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "update": {
            "id": "calendar.events.update",
            "path": "calendars/{calendarId}/events/{eventId}",
            "httpMethod": "PUT",
            "description": "Updates an event.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "conferenceDataVersion": {
                "type": "integer",
                "description": "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.",
                "format": "int32",
                "minimum": "0",
                "maximum": "1",
                "location": "query"
              },
              "eventId": {
                "type": "string",
                "description": "Event identifier.",
                "required": true,
                "location": "path"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "sendNotifications": {
                "type": "boolean",
                "description": "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.",
                "location": "query"
              },
              "sendUpdates": {
                "type": "string",
                "description": "Guests who should receive notifications about the event update (for example, title changes, etc.).",
                "enum": [
                  "all",
                  "externalOnly",
                  "none"
                ],
                "enumDescriptions": [
                  "Notifications are sent to all guests.",
                  "Notifications are sent to non-Google Calendar guests only.",
                  "No notifications are sent. For calendar migration tasks, consider using the Events.import method instead."
                ],
                "location": "query"
              },
              "supportsAttachments": {
                "type": "boolean",
                "description": "Whether API client performing operation supports event attachments. Optional. The default is False.",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId",
              "eventId"
            ],
            "request": {
              "$ref": "Event"
            },
            "response": {
              "$ref": "Event"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events"
            ]
          },
          "watch": {
            "id": "calendar.events.watch",
            "path": "calendars/{calendarId}/events/watch",
            "httpMethod": "POST",
            "description": "Watch for changes to Events resources.",
            "parameters": {
              "alwaysIncludeEmail": {
                "type": "boolean",
                "description": "Deprecated and ignored.",
                "location": "query"
              },
              "calendarId": {
                "type": "string",
                "description": "Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the \"primary\" keyword.",
                "required": true,
                "location": "path"
              },
              "eventTypes": {
                "type": "string",
                "description": "Event types to return. Optional. This parameter can be repeated multiple times to return events of different types. If unset, returns all event types.",
                "enum": [
                  "default",
                  "focusTime",
                  "fromGmail",
                  "outOfOffice",
                  "workingLocation"
                ],
                "enumDescriptions": [
                  "Regular events.",
                  "Focus time events.",
                  "Events from Gmail.",
                  "Out of office events.",
                  "Working location events."
                ],
                "repeated": true,
                "location": "query"
              },
              "iCalUID": {
                "type": "string",
                "description": "Specifies an event ID in the iCalendar format to be provided in the response. Optional. Use this if you want to search for an event by its iCalendar ID.",
                "location": "query"
              },
              "maxAttendees": {
                "type": "integer",
                "description": "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.",
                "default": "250",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "orderBy": {
                "type": "string",
                "description": "The order of the events returned in the result. Optional. The default is an unspecified, stable order.",
                "enum": [
                  "startTime",
                  "updated"
                ],
                "enumDescriptions": [
                  "Order by the start date/time (ascending). This is only available when querying single events (i.e. the parameter singleEvents is True)",
                  "Order by last modification time (ascending)."
                ],
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "privateExtendedProperty": {
                "type": "string",
                "description": "Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.",
                "repeated": true,
                "location": "query"
              },
              "q": {
                "type": "string",
                "description": "Free text search terms to find events that match these terms in the following fields:\n\n- summary \n- description \n- location \n- attendee's displayName \n- attendee's email \n- organizer's displayName \n- organizer's email \n- workingLocationProperties.officeLocation.buildingId \n- workingLocationProperties.officeLocation.deskId \n- workingLocationProperties.officeLocation.label \n- workingLocationProperties.customLocation.label \nThese search terms also match predefined keywords against all display title translations of working location, out-of-office, and focus-time events. For example, searching for \"Office\" or \"Bureau\" returns working location events of type officeLocation, whereas searching for \"Out of office\" or \"Abwesend\" returns out-of-office events. Optional.",
                "location": "query"
              },
              "sharedExtendedProperty": {
                "type": "string",
                "description": "Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.",
                "repeated": true,
                "location": "query"
              },
              "showDeleted": {
                "type": "boolean",
                "description": "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.",
                "location": "query"
              },
              "showHiddenInvitations": {
                "type": "boolean",
                "description": "Whether to include hidden invitations in the result. Optional. The default is False.",
                "location": "query"
              },
              "singleEvents": {
                "type": "boolean",
                "description": "Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nThere are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.\n\nThese are: \n- iCalUID \n- orderBy \n- privateExtendedProperty \n- q \n- sharedExtendedProperty \n- timeMin \n- timeMax \n- updatedMin All other query parameters should be the same as for the initial synchronization to avoid undefined behavior. If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              },
              "timeMax": {
                "type": "string",
                "description": "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.",
                "format": "date-time",
                "location": "query"
              },
              "timeMin": {
                "type": "string",
                "description": "Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.",
                "format": "date-time",
                "location": "query"
              },
              "timeZone": {
                "type": "string",
                "description": "Time zone used in the response. Optional. The default is the time zone of the calendar.",
                "location": "query"
              },
              "updatedMin": {
                "type": "string",
                "description": "Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.",
                "format": "date-time",
                "location": "query"
              }
            },
            "parameterOrder": [
              "calendarId"
            ],
            "request": {
              "$ref": "Channel",
              "parameterName": "resource"
            },
            "response": {
              "$ref": "Channel"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.events",
              "https://www.googleapis.com/auth/calendar.events.readonly",
              "https://www.googleapis.com/auth/calendar.readonly"
            ],
            "supportsSubscription": true
          }
        }
      },
      "freebusy": {
        "methods": {
          "query": {
            "id": "calendar.freebusy.query",
            "path": "freeBusy",
            "httpMethod": "POST",
            "description": "Returns free/busy information for a set of calendars.",
            "request": {
              "$ref": "FreeBusyRequest"
            },
            "response": {
              "$ref": "FreeBusyResponse"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly"
            ]
          }
        }
      },
      "settings": {
        "methods": {
          "get": {
            "id": "calendar.settings.get",
            "path": "users/me/settings/{setting}",
            "httpMethod": "GET",
            "description": "Returns a single user setting.",
            "parameters": {
              "setting": {
                "type": "string",
                "description": "The id of the user setting.",
                "required": true,
                "location": "path"
              }
            },
            "parameterOrder": [
              "setting"
            ],
            "response": {
              "$ref": "Setting"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly",
              "https://www.googleapis.com/auth/calendar.settings.readonly"
            ]
          },
          "list": {
            "id": "calendar.settings.list",
            "path": "users/me/settings",
            "httpMethod": "GET",
            "description": "Returns all user settings for the authenticated user.",
            "parameters": {
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "response": {
              "$ref": "Settings"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly",
              "https://www.googleapis.com/auth/calendar.settings.readonly"
            ],
            "supportsSubscription": true
          },
          "watch": {
            "id": "calendar.settings.watch",
            "path": "users/me/settings/watch",
            "httpMethod": "POST",
            "description": "Watch for changes to Settings resources.",
            "parameters": {
              "maxResults": {
                "type": "integer",
                "description": "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.",
                "format": "int32",
                "minimum": "1",
                "location": "query"
              },
              "pageToken": {
                "type": "string",
                "description": "Token specifying which result page to return. Optional.",
                "location": "query"
              },
              "syncToken": {
                "type": "string",
                "description": "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries.",
                "location": "query"
              }
            },
            "request": {
              "$ref": "Channel",
              "parameterName": "resource"
            },
            "response": {
              "$ref": "Channel"
            },
            "scopes": [
              "https://www.googleapis.com/auth/calendar",
              "https://www.googleapis.com/auth/calendar.readonly",
              "https://www.googleapis.com/auth/calendar.settings.readonly"
            ],
            "supportsSubscription": true
          }
        }
      }
    },
    "rootUrl": "https://www.googleapis.com/",
    "id": "calendar:v3",
    "protocol": "rest",
    "documentationLink": "https://developers.google.com/google-apps/calendar/firstapp",
    "batchPath": "batch/calendar/v3"
  }
